mod ddl_parser;
mod postgres_ddl;
mod postgres_enums;

use anyhow::Result;

pub fn get_database_structure(
    database_type: &str,
    connection_str: &str,
    schema_name: &str,
) -> Result<Vec<ddl_parser::Table>> {
    if database_type == "postgres" {
        let ddl = postgres_ddl::get_database_ddl(connection_str, schema_name)?;
        let enums = postgres_enums::get_database_enums(connection_str, schema_name)?;
        let database_definitions_result = ddl_parser::parse(ddl, &enums)?;

        Ok(database_definitions_result)
    } else {
        Ok(vec![])
    }
}
