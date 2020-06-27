use anyhow::Result;
use fake::faker::lorem::en::*;
use fake::faker::number::raw::*;
use fake::locales::*;
use fake::locales::{EN, ZH_TW};
use fake::{Dummy, Fake, Faker};
use rand::rngs::StdRng;
use rand::SeedableRng;

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
    let mut result = Vec::new();
    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let ref mut rng = StdRng::from_seed(seed);
    for _ in 0..1000 {
        let v: String = Word().fake_with_rng::<String, StdRng>(rng);
        result.push(v);
    }
    println!("random nested vec {:?}", result);
    vec![]
}
