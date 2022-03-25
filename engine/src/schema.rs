table! {
    activities (id) {
        id -> Uuid,
        activity_manager_id -> Nullable<Uuid>,
        actor_data -> Jsonb,
        data -> Jsonb,
        status -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    activity_managers (id) {
        id -> Uuid,
        process_state_id -> Nullable<Uuid>,
        #[sql_name = "type"]
        type_ -> Varchar,
        props -> Jsonb,
        parameters -> Jsonb,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    packages (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        code -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    processes (id) {
        id -> Uuid,
        workflow_id -> Nullable<Uuid>,
        blueprint_spec -> Jsonb,
        current_status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        current_state_id -> Nullable<Uuid>,
    }
}

table! {
    process_states (id) {
        id -> Uuid,
        process_id -> Nullable<Uuid>,
        engine_id -> Uuid,
        step_number -> Int4,
        node_id -> Varchar,
        next_node_id -> Nullable<Varchar>,
        bag -> Jsonb,
        actor_data -> Jsonb,
        external_input -> Nullable<Jsonb>,
        result -> Nullable<Jsonb>,
        error -> Nullable<Text>,
        status -> Varchar,
        time_elapsed -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    timers (id) {
        id -> Uuid,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        active -> Bool,
        resourse_type -> Varchar,
        resource_id -> Uuid,
        params -> Jsonb,
        fired_at -> Nullable<Timestamp>,
    }
}

table! {
    workflows (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        blueprint_spec -> Jsonb,
        blueprint_hash -> Text,
        version -> Int4,
        created_at -> Timestamp,
    }
}

joinable!(activities -> activity_managers (activity_manager_id));
joinable!(processes -> workflows (workflow_id));

allow_tables_to_appear_in_same_query!(
    activities,
    activity_managers,
    packages,
    processes,
    process_states,
    timers,
    workflows,
);
