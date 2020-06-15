mod data_processor;

use anyhow::Result;

pub fn get_data_type_by_name(name: &str, data_type:  &str) -> String {
    data_processor::get_data_type_by_name(name, data_type)
}
