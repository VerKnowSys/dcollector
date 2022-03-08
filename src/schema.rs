table! {
    sys_stats (time) {
        time -> Timestamp,
        name -> Nullable<Text>,
        kernel_version -> Nullable<Text>,
        os_version -> Nullable<Text>,
        host_name -> Nullable<Text>,
        processors -> Nullable<Int4>,
        total_memory -> Nullable<Int4>,
        used_memory -> Nullable<Int4>,
        total_swap -> Nullable<Int4>,
        used_swap -> Nullable<Int4>,
    }
}

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

allow_tables_to_appear_in_same_query!(
    sys_stats,
    ups_stats,
);
