use crate::postgres_enums::DatabaseEnum;
use anyhow::Result;
use sqlparser::ast::ColumnOption;
use sqlparser::ast::Statement;
use sqlparser::ast::TableConstraint;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub data_type: String,
    pub reference_table: Option<String>,
    pub is_not_null: bool,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub enum_values: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub fields: Vec<Field>,
}

pub fn parse(
    database_definitions: String,
    database_enums: &Vec<DatabaseEnum>,
) -> Result<Vec<Table>> {
    let mut tables = Vec::new();

    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, database_definitions)?;

    for table in ast {
        match table {
            Statement::CreateTable {
                name,
                columns,
                constraints,
                ..
            } => {
                let mut fields = Vec::new();
                for column in columns {
                    let mut reference_table: Option<String>;
                    let mut is_not_null = false;
                    let mut is_primary_key = false;
                    let mut is_unique = false;

                    let reference_table_option = constraints.iter().find_map(|d| match d {
                        TableConstraint::ForeignKey {
                            foreign_table,
                            columns,
                            ..
                        } if columns
                            .iter()
                            .any(|i| &i.replace("\"", "") == &column.name.to_string()) =>
                        {
                            Some(foreign_table.to_string())
                        }
                        _ => None,
                    });
                    let is_not_null_option = column.options.iter().find_map(|d| match d.option {
                        ColumnOption::NotNull => Some(true),
                        _ => None,
                    });
                    let is_primary_option = constraints.iter().find_map(|d| match d {
                        TableConstraint::Unique {
                            is_primary,
                            columns,
                            ..
                        } if columns
                            .iter()
                            .any(|i| &i.replace("\"", "") == &column.name.to_string())
                            && *is_primary =>
                        {
                            Some(true)
                        }
                        _ => None,
                    });
                    let is_unique_option = constraints.iter().find_map(|d| match d {
                        TableConstraint::Unique { columns, .. }
                            if columns
                                .iter()
                                .any(|i| &i.replace("\"", "") == &column.name.to_string()) =>
                        {
                            Some(true)
                        }
                        _ => None,
                    });

                    reference_table =
                        reference_table_option.map(|r| r.to_string().replace("\"", ""));
                    if let Some(ref result) = is_not_null_option {
                        is_not_null = *result;
                    }
                    if let Some(ref result) = is_primary_option {
                        is_primary_key = *result;
                    }
                    if let Some(ref result) = is_unique_option {
                        is_unique = *result;
                    }

                    let enum_values: Vec<String> = database_enums
                        .into_iter()
                        .filter(|e| e.name == column.data_type.to_string())
                        .map(|e| e.value.clone())
                        .collect();

                    let field = Field {
                        name: column.name.to_string(),
                        data_type: column.data_type.to_string(),
                        reference_table,
                        is_not_null,
                        is_primary_key,
                        is_unique,
                        enum_values: if enum_values.is_empty() {
                            None
                        } else {
                            Some(enum_values)
                        },
                    };

                    fields.push(field);
                }

                let table = Table {
                    name: name.to_string(),
                    schema: name.to_string(),
                    fields,
                };

                tables.push(table);
            }
            _ => println!("Something else"),
        }
    }

    Ok(tables)
}
