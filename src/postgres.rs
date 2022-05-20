use crate::{
    // disk_stats::host_name,
    models::DefaultWithTime,
    schema::{
        // disk_stats::dsl::disk_stats,
        // disk_stats::{dsl::disk_stats, time as disk_stats_time},
        disk_stats::dsl::disk_stats,
        // proc_stats::{dsl::proc_stats, time as proc_stats_time},
        proc_stats::dsl::proc_stats,
        // sys_stats::{dsl::sys_stats, time as sys_stats_time},
        sys_stats::dsl::sys_stats,
        // ups_stats::{dsl::ups_stats, time as ups_stats_time},
        ups_stats::dsl::ups_stats,
    },
    systeminfo::{disk_stats_entry, sys_process_entries, sys_stats_entry},
    ups::ups_stats_entry,
    *,
};
use diesel::{
    pg::PgConnection,
    prelude::*,
    result::{ConnectionError, Error},
};
use std::env;
use sysinfo::{System, SystemExt};


/// Establish connection with TimescaleDB
#[instrument]
pub fn establish_postgres_connection() -> Result<PgConnection, ConnectionError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}


/// Store all entries (Systat, UpsStat and ProcStat) in a single RDBMS transaction
pub fn store_entries(pg_connection: &PgConnection) -> Result<(), Error> {
    pg_connection.transaction(|| {
        // read data from system once:
        let mut sys = System::new_all();
        sys.refresh_all();

        // prevent from storing default values. Skip write to the DB in that case:

        // System stats (a single entry)
        let a_sys_stats_entry = sys_stats_entry(&sys);
        if a_sys_stats_entry != SysStat::default_skip_time(&a_sys_stats_entry) {
            diesel::insert_into(sys_stats)
                .values(a_sys_stats_entry)
                .get_result::<SysStat>(pg_connection)?;
        } else {
            debug!("Empty SysStat entry. Skipping DB store.");
        }

        // UPS stats (a single entry)
        let a_ups_stats_entry = ups_stats_entry();
        if a_ups_stats_entry != UpsStat::default_skip_time(&a_ups_stats_entry) {
            diesel::insert_into(ups_stats)
                .values(a_ups_stats_entry)
                .get_result::<UpsStat>(pg_connection)?;
        } else {
            debug!("Empty UpsStat entry. Skipping DB store.");
        }

        // Disk stats (multiple entries)
        let a_disk_stats_entries = disk_stats_entry(&sys)
            .into_iter()
            .filter_map(|entry| {
                if entry != DiskStat::default_skip_time(&entry) {
                    Some(entry)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if !a_disk_stats_entries.is_empty() {
            diesel::insert_into(disk_stats)
                .values(a_disk_stats_entries)
                .execute(pg_connection)?;
        } else {
            debug!("Empty DiskStat entry. Skipping DB store.");
        }

        // Processes stats (multiple entries)
        let a_proc_stats_entries = sys_process_entries(&sys)
            .into_iter()
            .filter_map(|entry| {
                if entry != ProcStat::default_skip_time(&entry) {
                    Some(entry)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if !a_proc_stats_entries.is_empty() {
            diesel::insert_into(proc_stats)
                .values(a_proc_stats_entries)
                .execute(pg_connection)?;
        } else {
            debug!("Empty ProcStat entry. Skipping DB store.");
        }

        Ok(())
    })
}
