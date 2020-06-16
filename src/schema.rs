table! {
    projects (id) {
        id -> Text,
        title -> Varchar,
        description -> Nullable<Text>,
        connection_string -> Nullable<Text>,
        ddl_schema -> Nullable<Text>,
        database_type -> Text,
        database_schema -> Nullable<Text>,
    }
}

table! {
    project_table_fields (id) {
        id -> Text,
        name -> Text,
        data_type -> Text,
        reference_table -> Nullable<Text>,
        fake_data_type -> Nullable<Text>,
        project_table_id -> Text,
        is_not_null -> Bool,
        is_primary_key -> Bool,
        is_unique -> Bool,
    }
}

table! {
    project_tables (id) {
        id -> Text,
        name -> Varchar,
        schema -> Varchar,
        project_id -> Text,
    }
}

joinable!(project_table_fields -> project_tables (project_table_id));
joinable!(project_tables -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    projects,
    project_table_fields,
    project_tables,
);
