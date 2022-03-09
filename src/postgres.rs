use crate::{
    models::{ProcStat, SysStat, UpsStat},
    schema::{
        proc_stats::{dsl::proc_stats, time as proc_stats_time},
        sys_stats::{dsl::sys_stats, time as sys_stats_time},
        ups_stats::{dsl::ups_stats, time as ups_stats_time},
    },
    systeminfo::{sys_process_entries, sys_stats_entry},
    ups_stats_entry,
};
use diesel::{
    pg::PgConnection,
    prelude::*,
    result::{ConnectionError, Error},
};
use std::env;


pub fn establish_postgres_connection() -> Result<PgConnection, ConnectionError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}


pub fn store_entries(pg_connection: &PgConnection) -> Result<(), Error> {
    pg_connection.transaction(|| {
        diesel::insert_into(sys_stats)
            .values(sys_stats_entry())
            .get_result::<SysStat>(pg_connection)?;

        diesel::insert_into(ups_stats)
            .values(ups_stats_entry())
            .get_result::<UpsStat>(pg_connection)?;

        // insert batch of entries
        diesel::insert_into(proc_stats)
            .values(sys_process_entries())
            .execute(pg_connection)?;

        Ok(())
    })
}


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

    let _results_procs = proc_stats
        .limit(amount as i64)
        .order(proc_stats_time.desc())
        .load::<ProcStat>(pg_connection)?;
    // for entry in &results_procs {
    //     println!("Proc: {}", entry);
    // }

    let len = results.len();
    println!(
        "Displaying {} UPS {}",
        len,
        if len > 1 { "entries" } else { "entry" }
    );
    for entry in &results {
        println!("{}", entry);
    }

    let len = results_system.len();
    println!(
        "Displaying {} system {}",
        len,
        if len > 1 { "entries" } else { "entry" }
    );
    for entry in &results_system {
        println!("{}", entry);
    }

    Ok(())
}
