table! {
    ups_stats (time) {
        time -> Timestamp,
        model -> Nullable<Text>,
        status -> Nullable<Text>,
        load -> Nullable<Int4>,
        input_frequency -> Nullable<Float8>,
        input_voltage -> Nullable<Float8>,
        battery_charge -> Nullable<Int4>,
        battery_voltage -> Nullable<Float8>,
    }
}
