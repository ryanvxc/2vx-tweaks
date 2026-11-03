use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use sysinfo::{CpuRefreshKind, Gpus, MemoryRefreshKind, Networks, RefreshKind, System};

/// How often the sampler thread records a sample.
pub const SAMPLE_INTERVAL: Duration = Duration::from_secs(1);
/// Ring buffer length: 5 minutes of history at one sample per second.
pub const HISTORY_CAPACITY: usize = 300;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MetricsSample {
    /// Unix epoch milliseconds.
    pub timestamp_ms: u64,
    pub cpu_percent: f32,
    pub per_core_percent: Vec<f32>,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub gpus: Vec<GpuSample>,
    pub network_rx_bytes_per_sec: u64,
    pub network_tx_bytes_per_sec: u64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GpuSample {
    pub model: Option<String>,
    /// Utilisation from PDH `\GPU Engine(*)\Utilization Percentage`, which sysinfo *sums*
    /// across all engines of the adapter. Clamped to 100; it can differ from Task Manager,
    /// which shows the busiest single engine. Per-engine sampling is planned for Phase 2.
    /// `None` when the counter is unavailable.
    pub usage_percent: Option<f32>,
    // No `memory_used_bytes`: sysinfo reads it from DXGI `QueryVideoMemoryInfo`, which reports the
    // *calling process's* VRAM usage, not system-wide usage, so it would always read ~0 here.
    pub memory_total_bytes: Option<u64>,
}

/// Fixed-capacity, time-ordered sample history.
#[derive(Debug, Default)]
pub struct History {
    samples: VecDeque<MetricsSample>,
}

impl History {
    pub fn push(&mut self, sample: MetricsSample) {
        if self.samples.len() == HISTORY_CAPACITY {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
    }

    /// Samples strictly newer than `since_ms`, or the whole history when `None`.
    pub fn since(&self, since_ms: Option<u64>) -> Vec<MetricsSample> {
        match since_ms {
            Some(t) => self
                .samples
                .iter()
                .filter(|s| s.timestamp_ms > t)
                .cloned()
                .collect(),
            None => self.samples.iter().cloned().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

/// Owns a background thread that samples utilisation once per second, so history
/// keeps accumulating whichever page is open and the UI never blocks on sysinfo.
pub struct MetricsSampler {
    history: Arc<Mutex<History>>,
}

impl MetricsSampler {
    pub fn start() -> Self {
        let history = Arc::new(Mutex::new(History::default()));
        let worker_history = Arc::clone(&history);

        let spawned = thread::Builder::new()
            .name("metrics-sampler".into())
            .spawn(move || sampler_loop(worker_history));
        if let Err(e) = spawned {
            tracing::error!(error = %e, "failed to start metrics sampler; live metrics disabled");
        }

        Self { history }
    }

    pub fn since(&self, since_ms: Option<u64>) -> Vec<MetricsSample> {
        // A poisoned lock means the sampler panicked mid-push; the data is still valid.
        let guard = self.history.lock().unwrap_or_else(|p| p.into_inner());
        guard.since(since_ms)
    }
}

fn sampler_loop(history: Arc<Mutex<History>>) {
    let mut system = match System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::nothing().with_usage())
            .with_memory(MemoryRefreshKind::nothing().with_ram()),
    ) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(error = %e, "metrics sampler could not initialise sysinfo");
            return;
        }
    };
    let mut networks = Networks::new_with_refreshed_list().ok();
    let mut gpus = Gpus::new_with_refreshed_list().ok();

    // CPU usage is a delta between two refreshes; prime the first reading.
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    let mut last_tick = Instant::now();

    loop {
        thread::sleep(SAMPLE_INTERVAL);

        system.refresh_cpu_usage();
        system.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());

        let elapsed = last_tick.elapsed().as_secs_f64().max(0.001);
        last_tick = Instant::now();

        let (rx, tx) = match networks.as_mut() {
            Some(nets) => {
                nets.refresh(true);
                nets.iter().fold((0u64, 0u64), |(rx, tx), (_, data)| {
                    (rx + data.received(), tx + data.transmitted())
                })
            }
            None => (0, 0),
        };

        let gpu_samples = match gpus.as_mut() {
            Some(gpus) => {
                gpus.refresh(true);
                gpus.iter()
                    .map(|g| GpuSample {
                        model: g.model().map(str::to_owned),
                        usage_percent: g.usage().map(|u| u.clamp(0.0, 100.0)),
                        memory_total_bytes: g.total_memory(),
                    })
                    .collect()
            }
            None => Vec::new(),
        };

        let sample = MetricsSample {
            timestamp_ms: now_ms(),
            cpu_percent: system.global_cpu_usage(),
            per_core_percent: system.cpus().iter().map(|c| c.usage()).collect(),
            memory_used_bytes: system.used_memory(),
            memory_total_bytes: system.total_memory(),
            gpus: gpu_samples,
            network_rx_bytes_per_sec: (rx as f64 / elapsed) as u64,
            network_tx_bytes_per_sec: (tx as f64 / elapsed) as u64,
        };

        history
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .push(sample);
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(ts: u64) -> MetricsSample {
        MetricsSample {
            timestamp_ms: ts,
            cpu_percent: 0.0,
            per_core_percent: vec![],
            memory_used_bytes: 0,
            memory_total_bytes: 0,
            gpus: vec![],
            network_rx_bytes_per_sec: 0,
            network_tx_bytes_per_sec: 0,
        }
    }

    #[test]
    fn history_is_bounded() {
        let mut h = History::default();
        for i in 0..(HISTORY_CAPACITY as u64 + 25) {
            h.push(sample(i));
        }
        assert_eq!(h.len(), HISTORY_CAPACITY);
        assert_eq!(h.since(None).first().unwrap().timestamp_ms, 25);
    }

    #[test]
    fn since_is_exclusive() {
        let mut h = History::default();
        (1..=5).for_each(|i| h.push(sample(i * 1000)));
        let newer: Vec<_> = h.since(Some(3000)).iter().map(|s| s.timestamp_ms).collect();
        assert_eq!(newer, vec![4000, 5000]);
    }

    #[test]
    fn sampler_produces_samples() {
        let sampler = MetricsSampler::start();
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL + SAMPLE_INTERVAL * 2);
        let samples = sampler.since(None);
        assert!(!samples.is_empty(), "sampler produced no samples");
        assert!(samples[0].memory_total_bytes > 0);
    }
}
