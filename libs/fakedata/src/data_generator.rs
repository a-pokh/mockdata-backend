use anyhow::Result;
use fake::faker::number::raw::*;
use fake::locales::*;
use fake::locales::{EN, ZH_TW};
use fake::Fake;

pub struct Table {
    pub name: String,
    pub schema: String,
    pub count: u32,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub data_type: Option<String>,
    pub reference_table: Option<String>,
    pub is_not_null: bool,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub enum_values: Option<Vec<String>>,
}

pub fn generate() -> Vec<String> {
    let val: String = Digit(EN).fake();
    println!("{:?}", val);

    // ^: 1-9, #: 0-9
    let val: String = NumberWithFormat(EN, "^###").fake();
    println!("{:?}", val);

    let val: String = NumberWithFormat(EN, "FLAT 0# ^#/F").fake();
    println!("{:?}", val);
    vec!["a".to_string()]
}
