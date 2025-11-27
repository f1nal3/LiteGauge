use sysinfo::{Disks, RefreshKind, System};

#[derive(Debug, serde::Serialize)]
pub struct Metrics {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub disk_usage: u64,
}

pub fn collect_metrics() -> Metrics {
    // Refresh CPU + memory
    let mut sys = System::new_with_specifics(RefreshKind::everything());
    sys.refresh_all();

    // CPU usage
    let avg_cpu = sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .sum::<f32>() / sys.cpus().len() as f32;

    // Disk usage via Disks API
    let disks = Disks::new_with_refreshed_list();

    let disk_usage: u64 = disks
        .iter()
        .map(|d| d.total_space() - d.available_space())
        .sum();

    Metrics {
        cpu_usage: avg_cpu,
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        disk_usage,
    }
}
