use crate::constants::*;
use fake::faker::lorem::en::*;
use fake::faker::name::raw::*;
use fake::locales::*;
use uuid::Uuid;

pub struct GeneratorTable {
    pub name: String,
    pub schema: String,
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

struct FieldResult {
    name: String,
    values: Vec<String>,
}

pub fn generate(tables: Vec<GeneratorTable>) -> Vec<String> {
    let mut script_strings: Vec<String> = Vec::new();
    for table in tables {
        let mut generated_table_data: Vec<FieldResult> = Vec::new();
        let mut column_names = String::new();

        for field in table.fields {
            generated_table_data.push(FieldResult {
                name: field.name.clone(),
                values: generate_stub(&field, table.count),
            });
        }

        for n in 0..table.count {
            let mut columns: Vec<String> = Vec::new();
            let mut values: Vec<String> = Vec::new();
            for table_item in &generated_table_data {
                let value = &table_item.values[n as usize];
                columns.push(table_item.name.clone());
                values.push(value.clone());
            }
            script_strings.push(format!(
                r#"insert into ({}) {}"#,
                columns.join(", "),
                values.join(", ")
            ));
        }

        println!("{:#?}", script_strings);
    }
    script_strings
}

fn generate_stub(field: &GeneratorField, count: u32) -> Vec<String> {
    match &field.data_type {
        Some(data_type) => match data_type.as_str() {
            ID_CUID => {
                let my_uuid: String = Uuid::new_v4().to_string();
            }
            _ => return vec![],
        },
        None => return vec![],
    }
    fake::vec![String as Name(EN); count as usize]
}
