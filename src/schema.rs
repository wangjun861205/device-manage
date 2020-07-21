table! {
    component (id) {
        id -> Integer,
        subsystem_id -> Integer,
        name -> Varchar,
        model -> Varchar,
        maintain_interval -> Integer,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    component_info (id) {
        id -> Integer,
        name -> Varchar,
        model -> Varchar,
        maintain_interval -> Integer,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    device (id) {
        id -> Integer,
        name -> Varchar,
        model -> Varchar,
        maintain_interval -> Integer,
        unicode -> Varchar,
        last_start_at -> Nullable<Datetime>,
        last_stop_at -> Nullable<Datetime>,
        total_duration -> Integer,
        status -> Varchar,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    device_info (id) {
        id -> Integer,
        name -> Varchar,
        model -> Varchar,
        maintain_interval -> Integer,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    deviceinfo_subsysteminfo (id) {
        id -> Integer,
        device_info_id -> Integer,
        subsystem_info_id -> Integer,
    }
}

table! {
    subsystem (id) {
        id -> Integer,
        device_id -> Integer,
        name -> Varchar,
        maintain_interval -> Integer,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    subsystem_info (id) {
        id -> Integer,
        name -> Varchar,
        maintain_interval -> Integer,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    subsysteminfo_componentinfo (id) {
        id -> Integer,
        subsystem_info_id -> Integer,
        component_info_id -> Integer,
        quantity -> Integer,
    }
}

joinable!(component -> subsystem (subsystem_id));
joinable!(deviceinfo_subsysteminfo -> device_info (device_info_id));
joinable!(deviceinfo_subsysteminfo -> subsystem_info (subsystem_info_id));
joinable!(subsystem -> device (device_id));
joinable!(subsysteminfo_componentinfo -> component_info (component_info_id));
joinable!(subsysteminfo_componentinfo -> subsystem_info (subsystem_info_id));

allow_tables_to_appear_in_same_query!(
    component,
    component_info,
    device,
    device_info,
    deviceinfo_subsysteminfo,
    subsystem,
    subsystem_info,
    subsysteminfo_componentinfo,
);
