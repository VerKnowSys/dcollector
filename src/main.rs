#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

pub mod models;
pub mod postgres;
pub mod schema;
pub mod ups;


use std::{env, thread, time::Duration};

use crate::{
    models::UpsStat,
    postgres::{establish_postgres_connection, print_entries, store_ups_entry},
    ups::ups_stats_entry,
};
use dotenv::dotenv;


fn main() {
    dotenv().ok();

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
