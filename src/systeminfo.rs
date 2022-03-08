use crate::models::SysStat;
use std::time::SystemTime;
use sysinfo::{ProcessExt, System, SystemExt};


pub fn sys_stats_entry() -> SysStat {
    let mut sys = System::new_all();
    sys.refresh_all();

    SysStat {
        time: SystemTime::now(),
        name: sys.name(),
        kernel_version: sys.kernel_version(),
        os_version: sys.os_version(),
        host_name: sys.host_name(),
        processors: Some(sys.processors().len() as i32),

        total_memory: Some(sys.total_memory() as i32),
        used_memory: Some(sys.used_memory() as i32),
        total_swap: Some(sys.total_swap() as i32),
        used_swap: Some(sys.used_swap() as i32),
    }
}
