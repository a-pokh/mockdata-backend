mod postgres_ddl;
mod ddl_parser;
//mod data_processor;

use anyhow::Result;

pub fn get_database_structure() -> Result<Vec<ddl_parser::Table>> {
    let connection_str = "postgresql://postgres@localhost:5432/stage_gtt";
    let schema_name = "public";

    let ddl = postgres_ddl::get_database_ddl(connection_str, schema_name)?;
    let database_definitions_result = ddl_parser::parse(ddl)?;

    // if let Ok(database_definitions) = database_definitions_result {
    //     //println!("{:#?}", data_processor::get_data_types());
    // }
    
    Ok(database_definitions_result)
}
