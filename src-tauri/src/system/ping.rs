//! One ICMP echo to one of two fixed targets, only ever because the user asked. A bufferbloat test
//! is a run of these: one a second to each target until it ends. Besides update checks this is the
//! only thing the app sends over a network, and it carries nothing but ping's own filler
//! (docs/security.md). The target is a name, so a command can't choose an address.

use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::{HANDLE, NO_ERROR};
use windows::Win32::NetworkManagement::IpHelper::{
    FreeMibTable, GetBestRoute2, GetIpForwardTable2, ICMP_ECHO_REPLY, IP_SUCCESS, IcmpCloseHandle,
    IcmpCreateFile, IcmpSendEcho2, MIB_IPFORWARD_ROW2, MIB_IPFORWARD_TABLE2,
};
use windows::Win32::Networking::WinSock::{AF_INET, SOCKADDR_INET};
use windows::Win32::System::IO::IO_STATUS_BLOCK;

use crate::error::{AppError, AppResult};

const INTERNET: Ipv4Addr = Ipv4Addr::new(1, 1, 1, 1);
/// The 32 bytes Windows' own `ping` sends.
const PAYLOAD: &[u8; 32] = b"abcdefghijklmnopqrstuvwabcdefghi";
const TIMEOUT_MS: u32 = 1000;
/// The documented minimum: a reply, the echoed payload, an ICMP error's 8 bytes, and room for the
/// driver's status block.
const REPLY_BYTES: usize =
    size_of::<ICMP_ECHO_REPLY>() + PAYLOAD.len() + 8 + size_of::<IO_STATUS_BLOCK>();

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PingTarget {
    /// This PC's default gateway, usually the router.
    Gateway,
    Internet,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PingResult {
    /// Empty when there's no default gateway to ask.
    pub address: String,
    /// `None` when no reply came within the timeout. A reply under a millisecond reads as 0.
    pub rtt_ms: Option<u32>,
}

pub fn ping(target: PingTarget) -> AppResult<PingResult> {
    let address = match target {
        PingTarget::Internet => Some(INTERNET),
        PingTarget::Gateway => default_gateway(),
    };
    let Some(address) = address else {
        return Ok(PingResult {
            address: String::new(),
            rtt_ms: None,
        });
    };
    Ok(PingResult {
        address: address.to_string(),
        rtt_ms: echo(address)?,
    })
}

/// The addresses in these structures are in network order, which is memory order.
fn wire(address: Ipv4Addr) -> u32 {
    u32::from_ne_bytes(address.octets())
}

/// The next hop towards the internet. Only reads the routing table: nothing is sent. A full-tunnel
/// VPN's route is on-link, with no hop to name, but the router is still there beneath it.
fn default_gateway() -> Option<Ipv4Addr> {
    best_route_hop()
        .and_then(gateway_at)
        .or_else(default_route_hop)
}

fn best_route_hop() -> Option<Ipv4Addr> {
    let mut destination = SOCKADDR_INET::default();
    destination.Ipv4.sin_family = AF_INET;
    destination.Ipv4.sin_addr.S_un.S_addr = wire(INTERNET);
    let mut route = MIB_IPFORWARD_ROW2::default();
    let mut source = SOCKADDR_INET::default();
    // SAFETY: every pointer is valid for the call.
    let status = unsafe { GetBestRoute2(None, 0, None, &destination, 0, &mut route, &mut source) };
    if status != NO_ERROR {
        return None;
    }
    // SAFETY: an IPv4 destination gives an IPv4 next hop, and the union's bytes are all integers.
    Some(hop_of(&route))
}

fn hop_of(route: &MIB_IPFORWARD_ROW2) -> Ipv4Addr {
    // SAFETY: an IPv4 route has an IPv4 next hop, and the union's bytes are all integers.
    let hop = unsafe { route.NextHop.Ipv4.sin_addr.S_un.S_addr };
    Ipv4Addr::from(hop.to_ne_bytes())
}

/// The best default route with a router on the end of it, from the whole IPv4 table.
fn default_route_hop() -> Option<Ipv4Addr> {
    let mut table: *mut MIB_IPFORWARD_TABLE2 = std::ptr::null_mut();
    // SAFETY: the out pointer is valid, and a successful call's table is freed below.
    if unsafe { GetIpForwardTable2(AF_INET, &mut table) } != NO_ERROR || table.is_null() {
        return None;
    }
    // SAFETY: `Table` is declared with one row but holds `NumEntries` of them, all inside the
    // allocation, which stays alive until FreeMibTable.
    let rows = unsafe {
        let first = (&raw const (*table).Table).cast::<MIB_IPFORWARD_ROW2>();
        std::slice::from_raw_parts(first, (*table).NumEntries as usize)
    };
    let hop = best_default_hop(rows.iter().map(|row| Route {
        prefix_length: row.DestinationPrefix.PrefixLength,
        metric: row.Metric,
        hop: hop_of(row),
    }));
    // SAFETY: the table came from GetIpForwardTable2 and `rows` isn't used past this point.
    unsafe { FreeMibTable(table.cast()) };
    hop
}

struct Route {
    prefix_length: u8,
    metric: u32,
    hop: Ipv4Addr,
}

/// Lowest metric wins, as it does for Windows among routes on one interface.
fn best_default_hop(routes: impl Iterator<Item = Route>) -> Option<Ipv4Addr> {
    routes
        .filter(|route| route.prefix_length == 0 && !route.hop.is_unspecified())
        .min_by_key(|route| route.metric)
        .map(|route| route.hop)
}

/// 0.0.0.0 means the destination is on this link, so there's no router in the way.
fn gateway_at(next_hop: Ipv4Addr) -> Option<Ipv4Addr> {
    (!next_hop.is_unspecified()).then_some(next_hop)
}

struct Icmp(HANDLE);

impl Drop for Icmp {
    fn drop(&mut self) {
        // SAFETY: an open handle this type owns, closed once.
        let _ = unsafe { IcmpCloseHandle(self.0) };
    }
}

/// The round trip in milliseconds, or `None` when nothing came back.
fn echo(address: Ipv4Addr) -> AppResult<Option<u32>> {
    // SAFETY: takes no arguments.
    let icmp = unsafe { IcmpCreateFile() }
        .map(Icmp)
        .map_err(|e| AppError::System(format!("can't send a ping: {}", e.message())))?;
    let mut reply = [0u64; REPLY_BYTES.div_ceil(8)];
    // SAFETY: the payload and reply buffers are valid for the sizes passed. Without an event or
    // callback the call blocks until a reply or the timeout, so both outlive it.
    let replies = unsafe {
        IcmpSendEcho2(
            icmp.0,
            None,
            None,
            None,
            wire(address),
            PAYLOAD.as_ptr().cast(),
            PAYLOAD.len() as u16,
            None,
            reply.as_mut_ptr().cast(),
            size_of_val(&reply) as u32,
            TIMEOUT_MS,
        )
    };
    if replies == 0 {
        return Ok(None);
    }
    // SAFETY: a non-zero count means the buffer starts with a reply, and it's aligned for one.
    let reply = unsafe { reply.as_ptr().cast::<ICMP_ECHO_REPLY>().read() };
    Ok((reply.Status == IP_SUCCESS).then_some(reply.RoundTripTime))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets_arrive_as_names() {
        let target: PingTarget = serde_json::from_str("\"gateway\"").unwrap();
        assert_eq!(target, PingTarget::Gateway);
        assert!(serde_json::from_str::<PingTarget>("\"8.8.8.8\"").is_err());
    }

    #[test]
    fn an_on_link_route_has_no_gateway() {
        assert_eq!(gateway_at(Ipv4Addr::UNSPECIFIED), None);
        let router = Ipv4Addr::new(192, 0, 2, 1);
        assert_eq!(gateway_at(router), Some(router));
    }

    #[test]
    fn a_tunnels_on_link_route_leaves_the_router_beneath_it() {
        let router = Ipv4Addr::new(192, 0, 2, 1);
        let spare = Ipv4Addr::new(192, 0, 2, 2);
        let route = |prefix_length, metric, hop| Route {
            prefix_length,
            metric,
            hop,
        };
        let table = [
            // The VPN: a lower metric, but no hop to name.
            route(0, 1, Ipv4Addr::UNSPECIFIED),
            // Not default routes, whatever their metric.
            route(24, 0, Ipv4Addr::new(192, 0, 2, 9)),
            route(32, 0, Ipv4Addr::new(192, 0, 2, 8)),
            route(0, 50, spare),
            route(0, 25, router),
        ];
        assert_eq!(best_default_hop(table.into_iter()), Some(router));
    }

    #[test]
    fn no_default_route_is_no_gateway() {
        let local = Route {
            prefix_length: 24,
            metric: 1,
            hop: Ipv4Addr::new(192, 0, 2, 1),
        };
        assert_eq!(best_default_hop([local].into_iter()), None);
        assert_eq!(best_default_hop(std::iter::empty()), None);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn the_reply_buffer_has_room_for_the_status_block() {
        assert_eq!(size_of::<IO_STATUS_BLOCK>(), 16);
        assert_eq!(
            REPLY_BYTES,
            size_of::<ICMP_ECHO_REPLY>() + PAYLOAD.len() + 8 + 16
        );
    }

    #[test]
    fn reading_the_route_table_works_here() {
        // Whatever this PC's network looks like, the walk has to come back with a real hop or none.
        assert_ne!(default_route_hop(), Some(Ipv4Addr::UNSPECIFIED));
    }

    #[test]
    fn addresses_go_out_in_network_order() {
        assert_eq!(wire(Ipv4Addr::new(1, 2, 3, 4)).to_ne_bytes(), [1, 2, 3, 4]);
    }
}
