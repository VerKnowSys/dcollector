use crate::*;
use serde_json::Value;
use std::{
    process::{Command, Stdio},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sysinfo::{CpuExt, ProcessExt, System, SystemExt};


/// Read and fill SysStat entry with system stats
#[instrument]
pub fn sys_stats_entry(sys: &System) -> SysStat {
    let cpu_cores = sys.physical_core_count().unwrap_or(1);
    let cpu_usage =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpu_cores as f32;
    let load_avg = sys.load_average();

    SysStat {
        time: SystemTime::now(),
        name: sys.name(),
        kernel_version: sys.kernel_version(),
        os_version: sys.os_version(),
        host_name: sys.host_name(),

        cpu_usage: Some(cpu_usage),
        load_one: Some(load_avg.one),
        load_five: Some(load_avg.five),
        load_fifteen: Some(load_avg.fifteen),

        processors: Some(cpu_cores as i32),

        total_memory: Some(sys.total_memory() as i64),
        used_memory: Some(sys.used_memory() as i64),
        total_swap: Some(sys.total_swap() as i64),
        used_swap: Some(sys.used_swap() as i64),
    }
}


/// Read and fill SysStat entry with stats from user processes
#[instrument]
pub fn sys_process_entries(sys: &System) -> Vec<ProcStat> {
    sys.processes()
        .values()
        .map(|process| {
            // Sleep 10ms to avoid time PK duplication with a lot of processes running in the system:
            thread::sleep(Duration::from_millis(10));

            let maybe_time = UNIX_EPOCH + Duration::from_secs(process.start_time());
            let start_time = if maybe_time == UNIX_EPOCH {
                // if the time is the same as UNIX_EPOCH it means that the process is short lived
                None
            } else {
                Some(maybe_time)
            };
            let name = process.name().to_string();
            let cmd = process.cmd().join(" ");
            let exec = process.exe().display().to_string();
            let exe = if exec.is_empty() {
                name.to_owned()
            } else {
                exec
            };
            let disk_usage = process.disk_usage();

            ProcStat {
                time: SystemTime::now(),
                host_name: sys.host_name(),
                exe: Some(exe),
                cmd: Some(cmd),
                name: Some(name),
                disk_read: Some(disk_usage.read_bytes as i64),
                disk_read_total: Some(disk_usage.total_read_bytes as i64),
                disk_written: Some(disk_usage.written_bytes as i64),
                disk_written_total: Some(disk_usage.total_written_bytes as i64),
                cpu_usage: Some(process.cpu_usage()),
                rss: Some(process.memory() as i64),
                status: Some(process.status().to_string()),
                start_time,
            }
        })
        .collect()
}


#[instrument]
/// Reads disks from sysctl on FreeBSD
fn read_devices_list() -> Vec<String> {
    match Command::new("sysctl")
        .args(["-n", "kern.disks"])
        .stdin(Stdio::null())
        .output()
    {
        Ok(output) => {
            let sysctl_disks_raw = String::from_utf8_lossy(&output.stdout).to_string();
            sysctl_disks_raw
                .split_whitespace()
                .filter_map(|dsk| {
                    if !dsk.starts_with("flash") && !dsk.starts_with("mmc") {
                        Some(format!("/dev/{dsk}"))
                    } else {
                        None
                    }
                })
                .collect()
        }
        Err(_er) => {
            vec![]
        }
    }
}


#[instrument]
/// Read and fill DiskStat entry with stats from the disks
pub fn disk_stats_entry(sys: &System) -> Vec<DiskStat> {
    read_devices_list()
        .into_iter()
        .filter_map(|disk_device| {
            let the_command = Command::new("smartctl")
                .args(["-j", "-f", "brief", "-A", &disk_device])
                .stdin(Stdio::null())
                .output();

            match the_command {
                Ok(output) => {
                    let data = String::from_utf8_lossy(&output.stdout).to_string();
                    let smartctl_obj: Value =
                        serde_json::from_str(data.as_str()).unwrap_or_default();
                    trace!(
                        "smartctl command successful, the parsed object: {smartctl_obj:#?}"
                    );

                    let mut disk_stat = DiskStat {
                        name: Some(disk_device),
                        host_name: sys.host_name(),
                        temperature: Some(
                            smartctl_obj["temperature"]["current"]
                                .as_f64()
                                .unwrap_or(0.0),
                        ),
                        ..DiskStat::default()
                    };

                    for attr in smartctl_obj["ata_smart_attributes"]["table"]
                        .as_array()
                        .expect("ata_smart_attributes.table should be retrievable")
                    {
                        if attr["name"] == "Seek_Error_Rate" {
                            // seek_error_rate => Seek_Error_Rate
                            disk_stat.seek_error_rate =
                                Some(attr["raw"]["value"].as_i64().unwrap_or(0));
                        }

                        if attr["name"] == "Throughput_Performance" {
                            // throughput => Throughput_Performance
                            disk_stat.throughput =
                                Some(attr["raw"]["value"].as_i64().unwrap_or(0));
                        }

                        if attr["name"] == "Raw_Read_Error_Rate" {
                            // read_error_rate => Raw_Read_Error_Rate
                            disk_stat.read_error_rate =
                                Some(attr["raw"]["value"].as_i64().unwrap_or(0));
                        }

                        if attr["name"] == "UDMA_CRC_Error_Count" {
                            // crc_errors => UDMA_CRC_Error_Count
                            disk_stat.crc_errors =
                                Some(attr["raw"]["value"].as_i64().unwrap_or(0));
                        }

                        if attr["name"] == "Seek_Time_Performance" {
                            // seek_time => Seek_Time_Performance
                            disk_stat.seek_time =
                                Some(attr["raw"]["value"].as_i64().unwrap_or(0));
                        }
                    }
                    Some(disk_stat)
                }
                Err(err) => {
                    error!("smartctl failed with: {err}");
                    None
                }
            }
        })
        .collect()
}
