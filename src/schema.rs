// @generated automatically by Diesel CLI.

diesel::table! {
    disk_stats (time) {
        time -> Timestamp,
        name -> Nullable<Text>,
        temperature -> Nullable<Float8>,
        crc_errors -> Nullable<Int8>,
        seek_time -> Nullable<Int8>,
        seek_error_rate -> Nullable<Int8>,
        throughput -> Nullable<Int8>,
        read_error_rate -> Nullable<Int8>,
        host_name -> Nullable<Text>,
    }
}

diesel::table! {
    net_stats (time) {
        time -> Timestamp,
        netdev -> Text,
        packets_received -> Int8,
        total_packets_received -> Int8,
        packets_transmitted -> Int8,
        total_packets_transmitted -> Int8,
        received -> Int8,
        total_received -> Int8,
        transmitted -> Int8,
        total_transmitted -> Int8,
        transmitted_errors -> Int8,
        transmitted_total_errors -> Int8,
        received_errors -> Int8,
        received_total_errors -> Int8,
    }
}

diesel::table! {
    proc_stats (time) {
        time -> Timestamp,
        start_time -> Nullable<Timestamp>,
        exe -> Nullable<Text>,
        cmd -> Nullable<Text>,
        name -> Nullable<Text>,
        disk_read -> Nullable<Int8>,
        disk_read_total -> Nullable<Int8>,
        disk_written -> Nullable<Int8>,
        disk_written_total -> Nullable<Int8>,
        cpu_usage -> Nullable<Float4>,
        rss -> Nullable<Int8>,
        status -> Nullable<Text>,
        host_name -> Nullable<Text>,
    }
}

diesel::table! {
    sys_stats (time) {
        time -> Timestamp,
        name -> Nullable<Text>,
        kernel_version -> Nullable<Text>,
        os_version -> Nullable<Text>,
        host_name -> Nullable<Text>,
        processors -> Nullable<Int4>,
        total_memory -> Nullable<Int8>,
        used_memory -> Nullable<Int8>,
        total_swap -> Nullable<Int8>,
        used_swap -> Nullable<Int8>,
        load_one -> Nullable<Float8>,
        load_five -> Nullable<Float8>,
        load_fifteen -> Nullable<Float8>,
        cpu_usage -> Nullable<Float4>,
    }
}

diesel::table! {
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

diesel::allow_tables_to_appear_in_same_query!(
    disk_stats,
    net_stats,
    proc_stats,
    sys_stats,
    ups_stats,
);
