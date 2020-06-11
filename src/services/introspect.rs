

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub data_type: String,
    pub fake_data_type: String,
    pub reference_table: String,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub fields: Vec<Field>
}

pub fn generate_data() -> Vec<Table> {
    let table1 = Table {
        name: String::from("Test"),
        schema: String::from("public"),
        fields: vec![
            Field {
                name: String::from("id"),
                data_type: String::from("Text"),
                reference_table: String::from(""),
                fake_data_type: String::from("id"),
            },
            Field {
                name: String::from("name"),
                data_type: String::from("Text"),
                reference_table: String::from(""),
                fake_data_type: String::from("name"),
            },
            Field {
                name: String::from("user"),
                data_type: String::from("Text"),
                reference_table: String::from("User"),
                fake_data_type: String::from("id"),
            },
        ],
    };
    let table2 = Table {
        name: String::from("User"),
        schema: String::from("public"),
        fields: vec![
            Field {
                name: String::from("id"),
                data_type: String::from("Text"),
                reference_table: String::from(""),
                fake_data_type: String::from("id"),
            },
            Field {
                name: String::from("firstName"),
                data_type: String::from("Text"),
                reference_table: String::from(""),
                fake_data_type: String::from("name"),
            },
        ],
    };

    vec![table1, table2]
}