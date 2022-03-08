use crate::{
    schema::ups_stats::{dsl::ups_stats, time},
    ups_stats_entry, UpsStat,
};
use diesel::result::Error;
use diesel::{pg::PgConnection, prelude::*};
use std::env;


pub fn establish_postgres_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to: {}", database_url))
}


pub fn store_ups_entry(pg_connection: &PgConnection) -> Result<UpsStat, Error> {
    diesel::insert_into(ups_stats)
        .values(ups_stats_entry())
        .get_result::<UpsStat>(pg_connection)
}


pub fn print_entries(
    pg_connection: &PgConnection,
    amount: usize,
) -> Result<Vec<UpsStat>, Error> {
    let results = ups_stats
        // .filter(model.eq("1600 SINUS"))
        .limit(amount as i64)
        .order(time.desc())
        .load::<UpsStat>(pg_connection)?;

    println!("Displaying {} entries", results.len());
    for entry in &results {
        println!("{}", entry);
    }
    Ok(results)
}
