#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

pub mod models;
pub mod postgres;
pub mod schema;
pub mod ups;


use crate::{
    models::UpsStat,
    postgres::{establish_postgres_connection, print_entries, store_ups_entry},
    ups::ups_stats_entry,
};
use dotenv::dotenv;


fn main() {
    dotenv().ok();

    let pg_conn = establish_postgres_connection();
    store_ups_entry(&pg_conn)
        .and(print_entries(&pg_conn, 10))
        .expect("Processing should work properly");
}
