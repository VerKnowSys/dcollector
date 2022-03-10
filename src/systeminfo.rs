use crate::{
    models::{ProcStat, SysStat},
    *,
};
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sysinfo::{ProcessExt, ProcessorExt, System, SystemExt};


/// Read and fill SysStat entry with system stats
#[instrument]
pub fn sys_stats_entry(sys: &System) -> SysStat {
    let cpu = sys.global_processor_info();
    let load_avg = sys.load_average();

    SysStat {
        time: SystemTime::now(),
        name: sys.name(),
        kernel_version: sys.kernel_version(),
        os_version: sys.os_version(),
        host_name: sys.host_name(),

        cpu_usage: Some(cpu.cpu_usage()),
        load_one: Some(load_avg.one),
        load_five: Some(load_avg.five),
        load_fifteen: Some(load_avg.fifteen),

        processors: Some(sys.processors().len() as i32),

        total_memory: Some(sys.total_memory() as i32),
        used_memory: Some(sys.used_memory() as i32),
        total_swap: Some(sys.total_swap() as i32),
        used_swap: Some(sys.used_swap() as i32),
    }
}


/// Read and fill SysStat entry with stats from user processes
#[instrument]
pub fn sys_process_entries(sys: &System) -> Vec<ProcStat> {
    let mut processes = vec![];
    for process in sys.processes().values() {
        let disk_usage = process.disk_usage();

        let maybe_time = UNIX_EPOCH + Duration::from_secs(process.start_time());
        let start_time = if maybe_time == UNIX_EPOCH {
            // if the time is the same as UNIX_EPOCH it means that the process is short lived
            None
        } else {
            Some(maybe_time)
        };
        let proc_stat = ProcStat {
            time: SystemTime::now(),
            exe: Some(process.exe().to_string_lossy().to_string()),
            cmd: Some(process.cmd().join(" ")),
            name: Some(process.name().to_string()),
            disk_read: Some(disk_usage.read_bytes as i64),
            disk_read_total: Some(disk_usage.total_read_bytes as i64),
            disk_written: Some(disk_usage.written_bytes as i64),
            disk_written_total: Some(disk_usage.total_written_bytes as i64),
            cpu_usage: Some(process.cpu_usage()),
            rss: Some(process.memory() as i64),
            status: Some(process.status().to_string()),
            start_time,
        };
        processes.push(proc_stat);

        // Sleep 10ms to avoid time PK duplication with a lot of processes running in system:
        thread::sleep(Duration::from_millis(10));
    }
    processes
}
