//! "Dcollector" TimescaleDB agent.

use dcollector::{
    postgres::{establish_postgres_connection, store_entries},
    *,
};
use dotenv::dotenv;
use std::{env, thread, time::Duration};
use sysinfo::{System, SystemExt};
use tracing_subscriber::{fmt, EnvFilter};


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

    let sleep = env::var("SLEEP_SECONDS")
        .unwrap_or_else(|_| String::from("5"))
        .parse::<u64>()
        .unwrap_or(5);

    info!(
        "Starting dcollector, version: {}",
        env!("CARGO_PKG_VERSION")
    );

    // setup once per runtime:
    let mut system = System::new_all();
    let mut iteration = 0u128;
    loop {
        iteration += 1;
        debug!("Iteration #{iteration} is starting…");

        // refresh the data every iteration
        system.refresh_all();

        // Continously attempt to make connection with the configured TimescaleDB:
        let mut pg_conn = match establish_postgres_connection() {
            Ok(connection) => connection,
            Err(error) => {
                error!(
                    "Sleeping 5s while we experience TimescaleDB Connection Failure: {error}",
                );
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        match store_entries(&mut system, &mut pg_conn) {
            Ok(_) => debug!("Iteration #{iteration} was successful."),
            Err(error) => {
                error!("Iteration #{iteration} failed with error: {error}");
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        }
        thread::sleep(Duration::from_secs(sleep));
    }
}
