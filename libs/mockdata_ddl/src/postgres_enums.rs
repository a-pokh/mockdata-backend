use anyhow::Result;
use postgres::{Client, NoTls};
use std::fs;

pub struct DatabaseEnum {
    pub name: String,
    pub value: String,
}

// TODO: need more efficient script to query only enums of required schema
pub fn get_database_enums(connection_str: &str, schema_name: &str) -> Result<Vec<DatabaseEnum>> {
    let mut enum_definitions = Vec::new();
    let mut client = Client::connect(connection_str, NoTls)?;

    let sql_get_enums: &str =
        &fs::read_to_string("./libs/mockdata_ddl/src/sql/postgres_get_enums.sql")?;

    for row in client.query(sql_get_enums, &[])? {
        let enum_schema_name: &str = row.get(0);
        let enum_name: &str = row.get(1);
        let enum_value: &str = row.get(2);

        if enum_schema_name == schema_name {
            enum_definitions.push(DatabaseEnum {
                name: String::from(enum_name),
                value: String::from(enum_value),
            });
        }
    }

    Ok(enum_definitions)
}
