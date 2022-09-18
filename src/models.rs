use crate::*;

use chrono::{DateTime, Local, TimeZone};
use core::fmt;
use serde::Deserialize;


/// ProcStat holds one row of user processes with resources usage
#[derive(Debug, Clone, Deserialize, Insertable, Queryable, PartialEq)]
pub struct ProcStat {
    /// PK
    pub time: SystemTime,
    /// Holds hostname where process is running
    pub host_name: Option<String>,
    /// Holds time, when process started
    pub start_time: Option<SystemTime>,
    /// Holds abs path to executable
    pub exe: Option<String>,
    /// Holds executable full command line
    pub cmd: Option<String>,
    /// Hold process name
    pub name: Option<String>,
    /// Holds disk read since last refresh
    pub disk_read: Option<i64>,
    /// Holds total disk read since process started
    pub disk_read_total: Option<i64>,
    /// Holds disk written since last refresh
    pub disk_written: Option<i64>,
    /// Holds disk written since process started
    pub disk_written_total: Option<i64>,
    /// Holds cpu usage of the process
    pub cpu_usage: Option<f32>,
    /// Holds memory usage of the process
    pub rss: Option<i64>,
    /// Holds process status
    pub status: Option<String>,
}


impl Default for ProcStat {
    fn default() -> ProcStat {
        ProcStat {
            time: SystemTime::now(),
            host_name: None,
            start_time: None,
            exe: None,
            cmd: None,
            name: None,
            disk_read: None,
            disk_read_total: None,
            disk_written: None,
            disk_written_total: None,
            cpu_usage: None,
            rss: None,
            status: None,
        }
    }
}


/// SysStat holds one row of system stats
#[derive(Debug, Clone, Deserialize, Insertable, Queryable, PartialEq)]
pub struct SysStat {
    /// PK
    pub time: SystemTime,
    /// Holds system name
    pub name: Option<String>,
    /// Holds kernel version
    pub kernel_version: Option<String>,
    /// Holds system version
    pub os_version: Option<String>,
    /// Holds machine's host name
    pub host_name: Option<String>,
    /// Holds amount of processors on the machine
    pub processors: Option<i32>,
    /// Holds total memory available on the machine
    pub total_memory: Option<i64>,
    /// Holds memory allocated/ used on the machine
    pub used_memory: Option<i64>,
    /// Holds total swap available on the machnie
    pub total_swap: Option<i64>,
    /// Holds swap used on the machine
    pub used_swap: Option<i64>,
    /// Holds load average one-mins
    pub load_one: Option<f64>,
    /// Holds load average five-mins
    pub load_five: Option<f64>,
    /// Holds load average fifteen-mins
    pub load_fifteen: Option<f64>,
    /// Holds total cpu usage on the system
    pub cpu_usage: Option<f32>,
}


impl Default for SysStat {
    fn default() -> SysStat {
        SysStat {
            time: SystemTime::now(),
            name: None,
            kernel_version: None,
            os_version: None,
            host_name: None,
            processors: None,
            total_memory: None,
            used_memory: None,
            total_swap: None,
            used_swap: None,
            load_one: None,
            load_five: None,
            load_fifteen: None,
            cpu_usage: None,
        }
    }
}


/// upsStat holds one row of UPS data fetched from Nut server
#[derive(Debug, Clone, Deserialize, Insertable, Queryable, PartialEq)]
pub struct DiskStat {
    /// PK
    pub time: SystemTime,
    /// Holds the host name
    pub host_name: Option<String>,
    /// Holds the device name
    pub name: Option<String>,
    /// Holds the disk temperature
    pub temperature: Option<f64>,
    /// CRC errors counter
    pub crc_errors: Option<i64>,
    /// Time for device to seek
    pub seek_time: Option<i64>,
    /// Seek error rate
    pub seek_error_rate: Option<i64>,
    /// Throughput performance
    pub throughput: Option<i64>,
    /// Read error rate
    pub read_error_rate: Option<i64>,
}


impl Default for DiskStat {
    fn default() -> DiskStat {
        DiskStat {
            time: SystemTime::now(),
            host_name: None,
            name: None,
            temperature: None,
            crc_errors: None,
            seek_time: None,
            seek_error_rate: None,
            throughput: None,
            read_error_rate: None,
        }
    }
}


/// upsStat holds one row of UPS data fetched from Nut server
#[derive(Debug, Clone, Deserialize, Insertable, Queryable, PartialEq)]
pub struct UpsStat {
    /// PK
    pub time: SystemTime,
    /// Holds UPS model name
    pub model: Option<String>,
    /// Holds UPS status
    pub status: Option<String>,
    /// Holds UPS load
    pub load: Option<i32>,
    /// Holds UPS input frequency
    pub input_frequency: Option<f64>,
    /// Holds UPS input voltage
    pub input_voltage: Option<f64>,
    /// Holds UPS battery charge
    pub battery_charge: Option<i32>,
    /// Holds UPS battery voltage
    pub battery_voltage: Option<f64>,
}


impl Default for UpsStat {
    fn default() -> UpsStat {
        UpsStat {
            time: SystemTime::now(),
            model: None,
            status: None,
            load: None,
            input_frequency: None,
            input_voltage: None,
            battery_charge: None,
            battery_voltage: None,
        }
    }
}


/// Convert SystemTime to chrono DateTime
#[instrument]
fn system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    Local.timestamp(sec, nsec)
}


impl Display for DiskStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {name}, Temperature: {temperature}, CRC Errors: {crc_errors}, Seek Time: {seek_time}, Seek Error Rate: {seek_error_rate}, Throughput: {throughput}, Read Error Rate: {read_error_rate}",
            name = self.name.clone().unwrap_or_default(),
            temperature = self.temperature.unwrap_or_default(),
            crc_errors = self.crc_errors.unwrap_or_default(),
            seek_time = self.seek_time.unwrap_or_default(),
            seek_error_rate = self.seek_error_rate.unwrap_or_default(),
            throughput = self.throughput.unwrap_or_default(),
            read_error_rate = self.read_error_rate.unwrap_or_default(),
        )
    }
}


impl Display for ProcStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start_time_str = if let Some(a_start_time) = self.start_time {
            system_time_to_date_time(a_start_time).to_string()
        } else {
            String::new()
        };
        write!(
            f,
            "Name: {name}, Exe: {exe}, Cmd: {cmd}, Status: {status}, Start time: {start_time}, CPU Usage: {cpu_usage}, Resident Memory: {rss}KiB, Disk Read: {disk_read} / {disk_read_total}, Disk Write: {disk_written} / {disk_written_total},",
            exe = self.exe.clone().unwrap_or_default(),
            cmd = self.cmd.clone().unwrap_or_default(),
            name = self.name.clone().unwrap_or_default(),
            disk_read = self.disk_read.unwrap_or_default(),
            disk_read_total = self.disk_read_total.unwrap_or_default(),
            disk_written = self.disk_written.unwrap_or_default(),
            disk_written_total = self.disk_written_total.unwrap_or_default(),
            cpu_usage = self.cpu_usage.unwrap_or_default(),
            rss = self.rss.unwrap_or_default(),
            status = self.status.clone().unwrap_or_default(),
            start_time = start_time_str,
        )
    }
}


impl Display for SysStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Time: {}, Name: {}, CPU usage: {}, Load: {} {} {}, Kernel version: {}, OS vesion: {}, Host name: {}, Processors: {}, Total memory: {}KiB, Used memory: {}KiB, Total swap: {}KiB, Used swap: {}KiB",
            system_time_to_date_time(self.time),
            self.name.clone().unwrap_or_default(),
            self.cpu_usage.unwrap_or_default(),
            self.load_one.unwrap_or_default(),
            self.load_five.unwrap_or_default(),
            self.load_fifteen.unwrap_or_default(),
            self.kernel_version.clone().unwrap_or_default(),
            self.os_version.clone().unwrap_or_default(),
            self.host_name.clone().unwrap_or_default(),
            self.processors.unwrap_or_default(),
            self.total_memory.unwrap_or_default(),
            self.used_memory.unwrap_or_default(),
            self.total_swap.unwrap_or_default(),
            self.used_swap.unwrap_or_default(),
        )
    }
}


impl Display for UpsStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Time: {}, Model: {}, Status: {}, Load: {}, Input frequency: {}, Input voltage: {}, Battery charge: {}, Battery voltage: {}",
            system_time_to_date_time(self.time),
            self.model.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            self.load.unwrap_or_default(),
            self.input_frequency.unwrap_or_default(),
            self.input_voltage.unwrap_or_default(),
            self.battery_charge.unwrap_or_default(),
            self.battery_voltage.unwrap_or_default(),
        )
    }
}


/// Common trait to implement a Default for a type, but we wish to skip the "time" field
pub trait DefaultWithTime {
    /// Return type Default, without the "time" field
    fn default_skip_time(entry: &Self) -> Self;
}


impl DefaultWithTime for ProcStat {
    fn default_skip_time(entry: &Self) -> Self {
        Self {
            time: entry.time,
            ..Self::default()
        }
    }
}


impl DefaultWithTime for SysStat {
    fn default_skip_time(entry: &Self) -> Self {
        Self {
            time: entry.time,
            ..Self::default()
        }
    }
}


impl DefaultWithTime for UpsStat {
    fn default_skip_time(entry: &Self) -> Self {
        Self {
            time: entry.time,
            ..Self::default()
        }
    }
}


impl DefaultWithTime for DiskStat {
    fn default_skip_time(entry: &Self) -> Self {
        Self {
            time: entry.time,
            ..Self::default()
        }
    }
}
