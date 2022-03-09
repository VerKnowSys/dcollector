use crate::{
    schema::{proc_stats, sys_stats, ups_stats},
    *,
};
use chrono::{DateTime, Local, TimeZone};

use core::fmt;

use diesel::*;
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};


#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
pub struct ProcStat {
    pub time: SystemTime,
    pub start_time: Option<SystemTime>,
    pub exe: Option<String>,
    pub cmd: Option<String>,
    pub name: Option<String>,
    pub disk_read: Option<i64>,
    pub disk_read_total: Option<i64>,
    pub disk_written: Option<i64>,
    pub disk_written_total: Option<i64>,
    pub cpu_usage: Option<f32>,
    pub rss: Option<i64>,
    pub status: Option<String>,
}


#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
pub struct SysStat {
    pub time: SystemTime,
    pub name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub host_name: Option<String>,
    pub processors: Option<i32>,
    pub total_memory: Option<i32>,
    pub used_memory: Option<i32>,
    pub total_swap: Option<i32>,
    pub used_swap: Option<i32>,
    pub load_one: Option<f64>,
    pub load_five: Option<f64>,
    pub load_fifteen: Option<f64>,
    pub cpu_usage: Option<f32>,
}


#[derive(Debug, Clone, Deserialize, Insertable, Queryable)]
pub struct UpsStat {
    pub time: SystemTime,
    pub model: Option<String>,
    pub status: Option<String>,
    pub load: Option<i32>,
    pub input_frequency: Option<f64>,
    pub input_voltage: Option<f64>,
    pub battery_charge: Option<i32>,
    pub battery_voltage: Option<f64>,
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
