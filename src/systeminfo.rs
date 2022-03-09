use crate::models::SysStat;
use std::time::SystemTime;
use sysinfo::{ProcessorExt, System, SystemExt};


pub fn sys_stats_entry() -> SysStat {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu = sys.global_processor_info();
    let load_avg = sys.load_average();
    let (one, five, fifteen) = (load_avg.one, load_avg.five, load_avg.fifteen);

    SysStat {
        time: SystemTime::now(),
        name: sys.name(),
        kernel_version: sys.kernel_version(),
        os_version: sys.os_version(),
        host_name: sys.host_name(),

        cpu_usage: Some(cpu.cpu_usage()),
        load_one: Some(one),
        load_five: Some(five),
        load_fifteen: Some(fifteen),

        processors: Some(sys.processors().len() as i32),

        total_memory: Some(sys.total_memory() as i32),
        used_memory: Some(sys.used_memory() as i32),
        total_swap: Some(sys.total_swap() as i32),
        used_swap: Some(sys.used_swap() as i32),
    }
}
