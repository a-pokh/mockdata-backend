use postgres::{Client, NoTls};
use std::fs;
use anyhow::Result;

pub fn get_database_ddl(connection_str: &str, schema_name: &str) -> Result<String> {
    let mut table_definitions = Vec::new();
    let mut client = Client::connect(connection_str, NoTls)?;

    let sql_create_function: &str = &fs::read_to_string("./libs/mockdata_ddl/src/sql/postgres_create_ddl_function.sql")?;
    let sql_drop_function: &str = &fs::read_to_string("./libs/mockdata_ddl/src/sql/postgres_drop_ddl_function.sql")?;

    client.execute(sql_create_function, &[])?;
    
    for row in client.query("SELECT * FROM describe_table($1, '.*');", &[&schema_name])? {
        let table_definition: &str = row.get(0);
        let table_definition = table_definition
            .replace("ON UPDATE", "")
            .replace("ON DELETE", "")
            .replace("CASCADE", "")
            .replace("SET NULL", "");
        let table_definition = table_definition.replace("timestamp(3)", "timestamp");
    
        table_definitions.push(table_definition.to_string());
    }

    client.execute(sql_drop_function, &[])?;

    Ok(table_definitions.join(" "))
}