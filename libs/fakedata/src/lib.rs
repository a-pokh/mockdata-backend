mod constants;
pub mod data_generator;
mod data_processor;

pub fn get_data_type(
    name: &str,
    data_type: &str,
    is_primary_key: bool,
    is_table_has_composite_pk: bool,
    is_reference: bool,
    is_enum: bool,
    is_unique: bool,
) -> Option<String> {
    data_processor::get_data_type(
        name,
        data_type,
        is_primary_key,
        is_table_has_composite_pk,
        is_reference,
        is_enum,
        is_unique,
    )
}

pub fn generate_data(tables: Vec<data_generator::GeneratorTable>) -> Vec<String> {
    data_generator::generate(tables)
}
