mod postgres_ddl;
mod ddl_parser;
//mod data_processor;

use anyhow::Result;

pub fn get_database_structure(database_type: &str, connection_str: &str, schema_name: &str) -> Result<Vec<ddl_parser::Table>> {
    if database_type == "postgres" {
        let ddl = postgres_ddl::get_database_ddl(connection_str, schema_name)?;
        let database_definitions_result = ddl_parser::parse(ddl)?;
    
        // if let Ok(database_definitions) = database_definitions_result {
        //     //println!("{:#?}", data_processor::get_data_types());
        // }
        
        Ok(database_definitions_result)
    } else {
        Ok(vec![])
    }
}
