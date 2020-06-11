table! {
    project_table_fields (id) {
        id -> Text,
        name -> Text,
        data_type -> Text,
        reference_table -> Nullable<Text>,
        fake_data_type -> Nullable<Text>,
        project_table_id -> Text,
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

table! {
    projects (id) {
        id -> Text,
        title -> Varchar,
        description -> Nullable<Text>,
        connection_string -> Nullable<Text>,
        ddl_schema -> Nullable<Text>,
    }
}

joinable!(project_table_fields -> project_tables (project_table_id));
joinable!(project_tables -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    project_table_fields,
    project_tables,
    projects,
);
