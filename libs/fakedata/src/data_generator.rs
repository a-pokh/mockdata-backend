use crate::constants::*;
use cuid::cuid;
use fake::faker::internet::raw::*;
use fake::faker::lorem::en::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::Rng;
use uuid::Uuid;

pub struct GeneratorTable {
    pub name: String,
    pub schema: Option<String>,
    pub count: u32,
    pub fields: Vec<GeneratorField>,
}

pub struct GeneratorField {
    pub name: String,
    pub data_type: Option<String>,
    pub reference_table: Option<String>,
    pub is_not_null: bool,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub enum_values: Option<Vec<String>>,
}

struct TableResult {
    name: String,
    field_results: Vec<FieldResult>,
}
struct FieldResult {
    name: String,
    values: Vec<String>,
}

pub fn generate(tables: Vec<GeneratorTable>) -> Vec<String> {
    let mut script_strings: Vec<String> = Vec::new();
    let mut tables_result: Vec<TableResult> = Vec::new();
    // firstly, we need generate fake data
    for table in &tables {
        let mut generated_table_data: Vec<FieldResult> = Vec::new();

        for field in &table.fields {
            generated_table_data.push(FieldResult {
                name: field.name.clone(),
                values: generate_data(&field, table.count),
            });
        }

        tables_result.push(TableResult {
            name: table.name.clone(),
            field_results: generated_table_data,
        });
    }
    // then propagate fake data, along with adding connections
    for table_result in &tables_result {
        let table = &tables.iter().find(|t| t.name == table_result.name).unwrap();

        for n in 0..table.count {
            let mut columns: Vec<String> = Vec::new();
            let mut values: Vec<String> = Vec::new();
            for table_item in &table_result.field_results {
                let value = &table_item.values[n as usize];
                let table_field = table
                    .fields
                    .iter()
                    .find(|f| f.name == table_item.name)
                    .unwrap();
                columns.push(table_item.name.clone());
                if table_field.reference_table.is_some() {
                    // get ids array from referenced table
                    // TODO: store column of referrenced table, to process not only ids
                    let ref_table_result = &tables_result
                        .iter()
                        .find(|t| t.name == table_field.reference_table.clone().unwrap())
                        .unwrap();
                    let ref_table = &tables
                        .iter()
                        .find(|t| t.name == table_field.reference_table.clone().unwrap())
                        .unwrap();
                    let ref_table_pk_field =
                        ref_table.fields.iter().find(|f| f.is_primary_key).unwrap();

                    let ref_field = ref_table_result
                        .field_results
                        .iter()
                        .find(|f| f.name == ref_table_pk_field.name)
                        .unwrap();
                    // if field is unique - pick all ids from ref table
                    // if not unique - pick random
                    // in case of unique - ref table should have same count of genereted items
                    values.push(ref_field.values[n as usize].clone());
                } else {
                    values.push(value.clone());
                }
            }
            let table_schema: String = match &table.schema {
                Some(schema) => format!("{}.", schema.to_string()),
                None => String::from(""),
            };
            script_strings.push(format!(
                r#"insert into {}"{}"({}) values {}"#,
                table_schema,
                table.name,
                columns
                    .into_iter()
                    .map(|col: String| format!(r#""{}""#, col))
                    .collect::<Vec<String>>()
                    .join(", "),
                values.join(", ")
            ));
        }

        println!("{:#?}", script_strings);
    }
    script_strings
}

fn generate_data(field: &GeneratorField, count: u32) -> Vec<String> {
    match &field.data_type {
        Some(data_type) => match data_type.as_str() {
            ID_CUID => return generate_id(&data_type, count),
            NUMBER_INTEGER => return generate_number(&data_type, count),
            NUMBER_FLOAT => return generate_number(&data_type, count),
            DATE_TIMESTAMP => return generate_date(&data_type, count),
            BOOLEAN_BOOLEAN => return generate_bool(count),
            INTERNET_EMAIL => {
                return fake::vec![String as SafeEmail(EN); count as usize]
                    .into_iter()
                    .map(|val| format!(r#"'{}'"#, val))
                    .collect()
            }
            _ => {
                return fake::vec![String as Word(); count as usize]
                    .into_iter()
                    .map(|val| format!(r#"'{}'"#, val))
                    .collect()
            }
        },
        None => return vec![],
    }
}

fn generate_id(data_type: &String, count: u32) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    for n in 0..count {
        match data_type {
            _ => values.push(format!(r#""{}""#, cuid().unwrap())),
        }
    }
    values
}

fn generate_number(data_type: &String, count: u32) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    for n in 0..count {
        match data_type.as_str() {
            NUMBER_INTEGER => values.push(rand::thread_rng().gen_range(0, 1000).to_string()),
            NUMBER_FLOAT => values.push(
                rand::thread_rng()
                    .gen_range::<f32, f32, f32>(0.1, 1000.1)
                    .to_string(),
            ),
            _ => values.push(String::from("null")),
        }
    }
    values
}

fn generate_date(data_type: &String, count: u32) -> Vec<String> {
    use fake::faker::chrono::raw::*;
    match data_type.as_str() {
        DATE_TIMESTAMP => {
            return fake::vec![chrono::NaiveDateTime as DateTime(EN); count as usize]
                .into_iter()
                .map(|val| format!(r#"'{}'"#, val))
                .collect()
        }
        _ => return vec![String::from("null"); count as usize],
    }
}

fn generate_bool(count: u32) -> Vec<String> {
    use fake::faker::boolean::raw::*;
    return fake::vec![bool as Boolean(EN, 50); count as usize]
        .into_iter()
        .map(|val| format!(r#"{}"#, val))
        .collect();
}
