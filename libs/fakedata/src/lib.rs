mod data_generator;
mod data_processor;

pub fn get_data_type_by_name(name: &str, data_type: &str) -> Option<String> {
    data_processor::get_data_type_by_name(name, data_type, false, false, false, false, false)
}

pub fn generate_data() -> Vec<String> {
    data_generator::generate()
}
