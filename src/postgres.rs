use crate::{
    schema::{
        disk_stats::{dsl::disk_stats, time as disk_stats_time},
        proc_stats::{dsl::proc_stats, time as proc_stats_time},
        sys_stats::{dsl::sys_stats, time as sys_stats_time},
        ups_stats::{dsl::ups_stats, time as ups_stats_time},
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
        let default_sys_entry = SysStat {
            time: a_sys_stats_entry.time,
            ..SysStat::default()
        };
        if a_sys_stats_entry != default_sys_entry {
            diesel::insert_into(sys_stats)
                .values(a_sys_stats_entry)
                .get_result::<SysStat>(pg_connection)?;
        } else {
            debug!("Empty SysStat entry. Skipping DB store.");
        }

        // UPS stats (a single entry)
        let a_ups_stats_entry = ups_stats_entry();
        let default_ups_entry = UpsStat {
            time: a_ups_stats_entry.time,
            ..UpsStat::default()
        };
        if a_ups_stats_entry != default_ups_entry {
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
                let default_disk_entry = DiskStat {
                    time: entry.time,
                    ..DiskStat::default()
                };
                if entry != default_disk_entry {
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
                let default_proc_entry = ProcStat {
                    time: entry.time,
                    ..ProcStat::default()
                };
                if entry != default_proc_entry {
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


/// Print "amount" of entries from RDBMS
pub fn print_entries(pg_connection: &PgConnection, amount: usize) -> Result<(), Error> {
    let results = ups_stats
        // .filter(model.eq("1600 SINUS"))
        .limit(amount as i64)
        .order(ups_stats_time.desc())
        .load::<UpsStat>(pg_connection)?;

    let results_system = sys_stats
        .limit(amount as i64)
        .order(sys_stats_time.desc())
        .load::<SysStat>(pg_connection)?;

    let results_procs = proc_stats
        .limit(amount as i64)
        .order(proc_stats_time.desc())
        .load::<ProcStat>(pg_connection)?;

    let results_disks = disk_stats
        .limit(amount as i64)
        .order(disk_stats_time.desc())
        .load::<DiskStat>(pg_connection)?;

    let len = results_procs.len();
    debug!(
        "Displaying {} Disk {}",
        len,
        if len > 1 { "entries" } else { "entry" }
    );
    for entry in &results_disks {
        debug!("Disks: {}", entry);
    }

    let len = results_procs.len();
    debug!(
        "Displaying {} Process {}",
        len,
        if len > 1 { "entries" } else { "entry" }
    );
    for entry in &results_procs {
        debug!("Processes: {}", entry);
    }

    let len = results.len();
    debug!(
        "Displaying {} UPS {}",
        len,
        if len > 1 { "entries" } else { "entry" }
    );
    for entry in &results {
        debug!("UPS: {}", entry);
    }

    let len = results_system.len();
    debug!(
        "Displaying {} system {}",
        len,
        if len > 1 { "entries" } else { "entry" }
    );
    for entry in &results_system {
        debug!("System: {}", entry);
    }

    Ok(())
}
