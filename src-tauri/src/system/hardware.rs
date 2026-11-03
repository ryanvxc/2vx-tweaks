use serde::Serialize;
use sysinfo::{
    CpuRefreshKind, DiskKind, Disks, Gpus, MemoryRefreshKind, Motherboard, RefreshKind, System,
};

use super::os::{OsInfo, read_os_info};
use crate::error::{AppError, AppResult};

/// A one-off, point-in-time description of the machine. Collected on demand;
/// nothing here changes while the app runs except disk free space and uptime.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemSnapshot {
    pub hostname: Option<String>,
    pub uptime_secs: Option<u64>,
    pub os: OsInfo,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub gpus: Vec<GpuInfo>,
    pub disks: Vec<DiskInfo>,
    pub motherboard: Option<MotherboardInfo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    pub brand: String,
    pub vendor: String,
    pub physical_cores: Option<usize>,
    pub logical_cores: usize,
    /// As reported by Windows; typically the base clock, not the boost clock.
    pub frequency_mhz: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub swap_total_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuInfo {
    pub vendor: Option<String>,
    pub model: Option<String>,
    /// Dedicated video memory as reported by DXGI.
    pub dedicated_memory_bytes: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StorageKind {
    Ssd,
    Hdd,
    Unknown,
}

impl From<DiskKind> for StorageKind {
    fn from(kind: DiskKind) -> Self {
        match kind {
            DiskKind::SSD => StorageKind::Ssd,
            DiskKind::HDD => StorageKind::Hdd,
            _ => StorageKind::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub kind: StorageKind,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub is_removable: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MotherboardInfo {
    pub vendor: Option<String>,
    pub name: Option<String>,
}

/// Collects a full snapshot. Blocking — call from a worker thread, never the UI thread.
pub fn collect_snapshot() -> AppResult<SystemSnapshot> {
    let system = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::nothing().with_frequency())
            .with_memory(MemoryRefreshKind::everything()),
    )
    .map_err(|e| AppError::System(format!("system information unavailable: {e}")))?;

    let first_cpu = system.cpus().first();
    let cpu = CpuInfo {
        brand: first_cpu
            .map(|c| c.brand().trim().to_owned())
            .unwrap_or_default(),
        vendor: first_cpu
            .map(|c| c.vendor_id().to_owned())
            .unwrap_or_default(),
        physical_cores: System::physical_core_count().ok(),
        logical_cores: system.cpus().len(),
        frequency_mhz: first_cpu.map(|c| c.frequency()).unwrap_or(0),
    };

    let memory = MemoryInfo {
        total_bytes: system.total_memory(),
        swap_total_bytes: system.total_swap(),
    };

    // GPU and disk failures are non-fatal: the rest of the snapshot is still useful.
    let gpus = match Gpus::new_with_refreshed_list() {
        Ok(gpus) => gpus
            .iter()
            .map(|g| GpuInfo {
                vendor: g.vendor().map(str::to_owned),
                model: g.model().map(str::to_owned),
                dedicated_memory_bytes: g.total_memory(),
            })
            .collect(),
        Err(e) => {
            tracing::warn!(error = %e, "GPU enumeration failed");
            Vec::new()
        }
    };

    let disks = match Disks::new_with_refreshed_list() {
        Ok(disks) => disks
            .iter()
            .map(|d| DiskInfo {
                name: d.name().to_string_lossy().into_owned(),
                mount_point: d.mount_point().to_string_lossy().into_owned(),
                file_system: d.file_system().to_string_lossy().into_owned(),
                kind: d.kind().into(),
                total_bytes: d.total_space(),
                available_bytes: d.available_space(),
                is_removable: d.is_removable(),
            })
            .collect(),
        Err(e) => {
            tracing::warn!(error = %e, "disk enumeration failed");
            Vec::new()
        }
    };

    let motherboard = Motherboard::new().ok().map(|m| MotherboardInfo {
        vendor: m.vendor_name(),
        name: m.name(),
    });

    Ok(SystemSnapshot {
        hostname: System::host_name().ok(),
        uptime_secs: System::uptime().ok(),
        os: read_os_info(),
        cpu,
        memory,
        gpus,
        disks,
        motherboard,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_real_snapshot() {
        let snap = collect_snapshot().expect("snapshot");
        assert!(snap.cpu.logical_cores > 0);
        assert!(snap.memory.total_bytes > 0);
        assert!(!snap.disks.is_empty(), "expected at least one disk");

        let json = serde_json::to_value(&snap).unwrap();
        assert!(json["cpu"]["logicalCores"].is_number(), "camelCase fields");
    }
}
