#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

pub mod models;
pub mod postgres;
pub mod schema;
pub mod ups;


use lockfile::Lockfile;
use std::fs::remove_file;
use std::io::Write;
use std::process::exit;
use std::{env, process, thread, time::Duration};

use crate::{
    models::UpsStat,
    postgres::{establish_postgres_connection, print_entries, store_ups_entry},
    ups::ups_stats_entry,
};
use dotenv::dotenv;


fn main() {
    dotenv().ok();

    let pidfile = env::var("PID_FILE").unwrap_or_else(|_| String::from("dcollector.pid"));
    let mut lockfile = Lockfile::create(&pidfile)
        .unwrap_or_else(|_| panic!("Couldn't obtain lockfile: {}!", pidfile));
    let pidstr = format!("{}", process::id());
    lockfile
        .write_all(pidstr.as_bytes())
        .expect("Couldn't write pid to a lock file");
    ctrlc::set_handler(move || {
        remove_file(pidfile.clone()).unwrap_or_default();
        println!("Interrupted…");
        exit(1);
    })
    .expect("Error setting Ctrl-C handler");

    let sleep = env::var("SLEEP_SECONDS")
        .unwrap_or_else(|_| String::from("60"))
        .parse::<u64>()
        .unwrap_or(60);

    let pg_conn = establish_postgres_connection();
    loop {
        store_ups_entry(&pg_conn)
            .and(print_entries(&pg_conn, 1))
            .expect("Processing should work properly");
        thread::sleep(Duration::from_secs(sleep));
    }
}
