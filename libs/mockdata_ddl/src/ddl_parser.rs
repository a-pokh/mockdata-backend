use crate::postgres_enums::DatabaseEnum;
use anyhow::Result;
use regex::Regex;
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
    pub schema: Option<String>,
    pub fields: Vec<Field>,
}
impl Table {
    pub fn has_composite_primary_key(&self) -> bool {
        let count = &self.fields.iter().filter(|f| f.is_primary_key).count();

        return *count > 1 as usize;
    }
}

pub fn parse(
    database_definitions: String,
    database_enums: &Vec<DatabaseEnum>,
) -> Result<Vec<Table>> {
    let mut tables = Vec::new();
    let re = Regex::new(r#"""#).unwrap();

    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, &database_definitions)?;

    for table in &ast {
        match &table {
            Statement::CreateTable {
                name,
                columns,
                constraints,
                ..
            } => {
                println!("{:?}", name);
                let mut fields = Vec::new();
                for column in columns {
                    let reference_table: Option<String>;
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
                            .any(|i| &i.value.replace("\"", "") == &column.name.to_string()) =>
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
                            .any(|i| &i.value.replace("\"", "") == &column.name.to_string())
                            && *is_primary =>
                        {
                            Some(true)
                        }
                        _ => None,
                    });
                    let is_unique_option = constraints.iter().find_map(|d| match d {
                        TableConstraint::Unique { columns, .. }
                            if columns.iter().any(|i| {
                                &i.value.replace("\"", "") == &column.name.to_string()
                            }) =>
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
                // name contains schema.name
                let string_name = name.to_string();
                let mut split = string_name.split(".");
                let schema_and_name = split.collect::<Vec<&str>>();

                let table = Table {
                    name: schema_and_name[1].to_string(),
                    schema: Some(schema_and_name[0].to_string()),
                    fields,
                };

                tables.push(table);
            }
            _ => {}
        }
    }
    for table in &ast {
        match &table {
            Statement::CreateIndex {
                table_name,
                columns,
                unique,
                ..
            } => {
                let table_name = &table_name.to_string();
                let table_name_a = re.replace_all(table_name, "");
                if *unique {
                    let table = tables
                        .iter_mut()
                        .find(|t| t.name.trim().eq_ignore_ascii_case(&table_name_a.trim()));
                    if let Some(table) = table {
                        for field in &mut table.fields {
                            let exist = columns.iter().any(|c| c.value.to_string() == field.name);
                            if exist {
                                field.is_unique = true;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(tables)
}
