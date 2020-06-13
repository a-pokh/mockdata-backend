use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::ast::Statement;
use sqlparser::ast::TableConstraint;
use sqlparser::parser::Parser;
use anyhow::Result;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub data_type: String,
    pub fake_data_type: String,
    pub reference_table: String
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub fields: Vec<Field>
}

pub fn parse(database_definitions: String) -> Result<Vec<Table>>  {
    let mut tables = Vec::new();

    let dialect = PostgreSqlDialect {}; 
    let ast = Parser::parse_sql(&dialect, database_definitions)?;

    for table in ast {
        match table {
            Statement::CreateTable {name, columns, constraints, ..} => { 
                let mut fields = Vec::new();
                for column in columns {
                    let mut reference_table = "";
                    let found = constraints.iter().find_map(|d| match d {
                        TableConstraint::ForeignKey { foreign_table, columns, .. } if columns.iter().any(|i| &i.replace("\"", "")==&column.name.to_string()) =>  Some(foreign_table.to_string()),
                        _ => None,
                    });
                    
                    if let Some(ref references_found) = found {
                        reference_table = references_found;
                    }

                    let field = Field {
                        name: column.name.to_string(),
                        data_type: column.data_type.to_string(),
                        reference_table: reference_table.to_string().replace("\"", ""),
                        fake_data_type: String::from("")
                    };

                    fields.push(field);
                }

                let table = Table {
                    name: name.to_string(),
                    schema: name.to_string(),
                    fields,
                };

                tables.push(table);
            },
            _ => println!("Something else"),
        }
    }

    Ok(tables)
}
