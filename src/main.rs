//! "Dcollector" TimescaleDB agent.

#![forbid(unsafe_code)]
#![deny(
    missing_docs,
    unstable_features,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]


#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;


#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

/// RDBM models
pub mod models;
/// Postgres functions
pub mod postgres;
/// Autogenerated Diesel schema
#[allow(missing_docs)]
pub mod schema;
/// System info API
pub mod systeminfo;
/// UPS API
pub mod ups;

pub use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::{fmt, EnvFilter};

use lockfile::Lockfile;
use std::{
    env,
    fs::remove_file,
    io::Write,
    process::{self, exit},
    thread,
    time::Duration,
};


use crate::{
    models::UpsStat,
    postgres::{establish_postgres_connection, print_entries, store_entries},
    ups::ups_stats_entry,
};
use dotenv::dotenv;


/// Initialize logger and tracingformatter
#[instrument]
fn initialize() {
    let env_log = match EnvFilter::try_from_env("LOG") {
        Ok(env_value_from_env) => env_value_from_env,
        Err(_) => EnvFilter::from("info"),
    };
    fmt()
        .compact()
        .with_thread_names(false)
        .with_thread_ids(false)
        .with_ansi(true)
        .with_env_filter(env_log)
        .with_filter_reloading()
        .init();
}


/// main()
#[instrument]
fn main() {
    initialize();
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
        .unwrap_or_else(|_| String::from("10"))
        .parse::<u64>()
        .unwrap_or(10);

    info!(
        "Starting dcollector, version: {}",
        env!("CARGO_PKG_VERSION")
    );

    let mut iteration = 0u64;
    loop {
        iteration += 1;
        info!("Iteration #{} is starting…", iteration);
        // Continously attempt to make connection with the configured TimescaleDB:
        let pg_conn = match establish_postgres_connection() {
            Ok(connection) => connection,
            Err(error) => {
                error!(
                    "Sleeping 5s while we experience PostgreSQL TimescaleDB! Error: {}",
                    error
                );
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        match store_entries(&pg_conn).and(print_entries(&pg_conn, 1)) {
            Ok(_) => info!("Iteration #{} was successful.", iteration),
            Err(error) => {
                error!("Iteration #{} failed with error: {}", iteration, error);
                continue;
            }
        }
        thread::sleep(Duration::from_secs(sleep));
    }
}
