use crate::UpsStat;
use nut_client::{blocking::Connection as NutConnection, ConfigBuilder, Variable};
use std::{convert::TryInto, env, time::SystemTime};


pub fn ups_stats_entry() -> UpsStat {
    let nut_host = env::var("NUT_HOST").unwrap_or_else(|_| "vks0".to_string());
    let nut_ups = env::var("NUT_UPS").unwrap_or_else(|_| "eta".to_string());
    let nut_config = ConfigBuilder::new()
        .with_host((nut_host.clone(), 3493).try_into().unwrap_or_default())
        .with_debug(false) // Turn this on for debugging network chatter
        .build();

    match NutConnection::new(&nut_config) {
        Ok(mut nut_connection) => {
            UpsStat {
                time: SystemTime::now(),
                model: Some(
                    nut_connection
                        .get_var(&nut_ups, "ups.model")
                        .unwrap_or_else(|err| {
                            println!("Error: No UPS model available?: {}", err);
                            Variable::Other((String::from("ups.model"), String::new()))
                        })
                        .value(),
                ),
                status: Some(
                    nut_connection
                        .get_var(&nut_ups, "ups.status")
                        .unwrap_or_else(|err| {
                            println!("Error: No UPS status available?: {}", err);
                            Variable::Other((String::from("ups.status"), String::new()))
                        })
                        .value(),
                ),
                load: nut_connection
                    .get_var(&nut_ups, "ups.load")
                    .unwrap_or_else(|err| {
                        println!("Error: No UPS load available?: {}", err);
                        Variable::Other((String::from("ups.load"), String::new()))
                    })
                    .value()
                    .parse::<i32>()
                    .ok(),
                input_frequency: nut_connection
                    .get_var(&nut_ups, "input.frequency")
                    .unwrap_or_else(|err| {
                        println!("Error: No UPS input frequency available?: {}", err);
                        Variable::Other((String::from("input.frequency"), String::new()))
                    })
                    .value()
                    .parse::<f64>()
                    .ok(),
                input_voltage: nut_connection
                    .get_var(&nut_ups, "input.voltage")
                    .unwrap_or_else(|err| {
                        println!("Error: No UPS input voltage available?: {}", err);
                        Variable::Other((String::from("input.voltage"), String::new()))
                    })
                    .value()
                    .parse::<f64>()
                    .ok(),
                battery_charge: nut_connection
                    .get_var(&nut_ups, "battery.charge")
                    .unwrap_or_else(|err| {
                        println!("Error: No UPS battery charge available?: {}", err);
                        Variable::Other((String::from("battery.charge"), String::new()))
                    })
                    .value()
                    .parse::<i32>()
                    .ok(),
                battery_voltage: nut_connection
                    .get_var(&nut_ups, "battery.voltage")
                    .unwrap_or_else(|err| {
                        println!("Error: No UPS battery voltage available?: {}", err);
                        Variable::Other((String::from("battery.voltage"), String::new()))
                    })
                    .value()
                    .parse::<f64>()
                    .ok(),
            }
        }
        Err(error) => {
            println!(
                "Nut connection should work for UPS: {}@{}. Error: {}",
                nut_ups, nut_host, error
            );
            UpsStat {
                time: SystemTime::now(),
                model: None,
                status: None,
                load: None,
                input_frequency: None,
                input_voltage: None,
                battery_charge: None,
                battery_voltage: None,
            }
        }
    }
}
