// use crate::models::UpsData;
use crate::UpsStat;
use nut_client::blocking::Connection as NutConnection;
use nut_client::ConfigBuilder;
use std::convert::TryInto;
use std::env;
use std::time::SystemTime;


pub fn ups_stats_entry() -> UpsStat {
    let nut_host = env::var("NUT_HOST").unwrap_or_else(|_| "vks0".to_string());
    let nut_ups = env::var("NUT_UPS").unwrap_or_else(|_| "eta".to_string());
    let nut_config = ConfigBuilder::new()
        .with_host((nut_host, 3493).try_into().unwrap_or_default())
        .with_debug(false) // Turn this on for debugging network chatter
        .build();

    let mut nut_connection =
        NutConnection::new(&nut_config).expect("Nut server on vks0 should work");
    UpsStat {
        time: SystemTime::now(),
        model: Some(
            nut_connection
                .get_var(&nut_ups, "ups.model")
                .expect("No UPS model available?")
                .value(),
        ),
        status: Some(
            nut_connection
                .get_var(&nut_ups, "ups.status")
                .expect("No UPS status available?")
                .value(),
        ),
        load: nut_connection
            .get_var(&nut_ups, "ups.load")
            .expect("No UPS load available?")
            .value()
            .parse::<i32>()
            .ok(),
        input_frequency: nut_connection
            .get_var(&nut_ups, "input.frequency")
            .expect("No UPS input frequency available?")
            .value()
            .parse::<f64>()
            .ok(),
        input_voltage: nut_connection
            .get_var(&nut_ups, "input.voltage")
            .expect("No UPS input voltage available?")
            .value()
            .parse::<f64>()
            .ok(),
        battery_charge: nut_connection
            .get_var(&nut_ups, "battery.charge")
            .expect("No UPS battery charge available?")
            .value()
            .parse::<i32>()
            .ok(),
        battery_voltage: nut_connection
            .get_var(&nut_ups, "battery.voltage")
            .expect("No UPS battery voltage available?")
            .value()
            .parse::<f64>()
            .ok(),
    }
}
