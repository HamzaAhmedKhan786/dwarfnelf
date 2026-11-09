use serde::Serialize;
use sysinfo::{Disks, System};

#[derive(Serialize)]
pub struct DiskCapacity {
    total_bytes: u64,
    available_bytes: u64,
}

#[derive(Serialize)]
pub struct Hardware {
    pub logical_cpus: Option<usize>,
    pub total_ram_bytes: Option<u64>,
    pub available_ram_bytes: Option<u64>,
    architecture: &'static str,
    disks: Vec<DiskCapacity>,
    gpu_inventory_available: bool,
}

pub fn inspect() -> Hardware {
    let mut system = System::new();
    system.refresh_cpu_list(sysinfo::CpuRefreshKind::nothing());
    system.refresh_memory();
    Hardware {
        architecture: std::env::consts::ARCH,
        logical_cpus: (!system.cpus().is_empty()).then_some(system.cpus().len()),
        total_ram_bytes: (system.total_memory() > 0).then_some(system.total_memory()),
        available_ram_bytes: (system.total_memory() > 0).then_some(system.available_memory()),
        disks: Disks::new_with_refreshed_list()
            .iter()
            .map(|disk| DiskCapacity {
                total_bytes: disk.total_space(),
                available_bytes: disk.available_space(),
            })
            .collect(),
        gpu_inventory_available: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_consistent_capacity() {
        let hardware = inspect();
        assert!(hardware.logical_cpus.is_none_or(|count| count > 0));
        if let Some((total, available)) = hardware.total_ram_bytes.zip(hardware.available_ram_bytes)
        {
            assert!(available <= total);
        }
        assert!(hardware
            .disks
            .iter()
            .all(|disk| disk.available_bytes <= disk.total_bytes));
    }
}
