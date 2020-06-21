const LOREM_WORDS: &str = "words";
const LOREM_WORD: &str = "word";
const NAME_NAME: &str = "name";
const NAME_FIRST_NAME: &str = "first_name";
const NAME_LAST_NAME: &str = "last_name";
const NAME_TITLE: &str = "title";
const INTERNET_EMAIL: &str = "email";

#[derive(Debug)]
pub struct FakeDataType {
    name: String,
    category_name: String,
}

pub fn _get_data_types() -> Vec<FakeDataType> {
    let mut data_types = Vec::new();

    data_types.push(FakeDataType {
        name: String::from(LOREM_WORDS),
        category_name: String::from("lorem"),
    });
    data_types.push(FakeDataType {
        name: String::from(LOREM_WORD),
        category_name: String::from("lorem"),
    });
    data_types.push(FakeDataType {
        name: String::from(NAME_FIRST_NAME),
        category_name: String::from("name"),
    });
    data_types.push(FakeDataType {
        name: String::from(NAME_LAST_NAME),
        category_name: String::from("name"),
    });
    data_types.push(FakeDataType {
        name: String::from(NAME_NAME),
        category_name: String::from("name"),
    });
    data_types.push(FakeDataType {
        name: String::from(NAME_TITLE),
        category_name: String::from("name"),
    });
    data_types.push(FakeDataType {
        name: String::from("number_with_format"),
        category_name: String::from("number"),
    });
    data_types.push(FakeDataType {
        name: String::from("boolean"),
        category_name: String::from("boolean"),
    });
    data_types.push(FakeDataType {
        name: String::from(INTERNET_EMAIL),
        category_name: String::from("internet"),
    });
    data_types.push(FakeDataType {
        name: String::from("username"),
        category_name: String::from("internet"),
    });
    data_types.push(FakeDataType {
        name: String::from("password"),
        category_name: String::from("internet"),
    });
    data_types.push(FakeDataType {
        name: String::from("ipV4"),
        category_name: String::from("internet"),
    });
    data_types.push(FakeDataType {
        name: String::from("ipV6"),
        category_name: String::from("internet"),
    });
    data_types.push(FakeDataType {
        name: String::from("color"),
        category_name: String::from("internet"),
    });
    data_types.push(FakeDataType {
        name: String::from("company_name"),
        category_name: String::from("company"),
    });
    data_types.push(FakeDataType {
        name: String::from("profession"),
        category_name: String::from("company"),
    });
    data_types.push(FakeDataType {
        name: String::from("industry"),
        category_name: String::from("company"),
    });
    data_types.push(FakeDataType {
        name: String::from("phone_number"),
        category_name: String::from("phone_number"),
    });
    data_types.push(FakeDataType {
        name: String::from("time"),
        category_name: String::from("date_time"),
    });
    data_types.push(FakeDataType {
        name: String::from("date"),
        category_name: String::from("date_time"),
    });
    data_types.push(FakeDataType {
        name: String::from("date_time"),
        category_name: String::from("date_time"),
    });
    data_types.push(FakeDataType {
        name: String::from("file_path"),
        category_name: String::from("file_system"),
    });
    data_types.push(FakeDataType {
        name: String::from("file_name"),
        category_name: String::from("file_system"),
    });
    data_types.push(FakeDataType {
        name: String::from("file_extension"),
        category_name: String::from("file_system"),
    });

    data_types
}

pub fn get_data_type_by_name(name: &str, data_type: &str) -> Option<String> {
    let mut retval_str: &str = "";

    if check_if_email(name, data_type) {
        retval_str = INTERNET_EMAIL;
    } else if check_if_first_name(name, data_type) {
        retval_str = NAME_FIRST_NAME;
    } else if check_if_last_name(name, data_type) {
        retval_str = NAME_LAST_NAME;
    } else if check_if_name(name, data_type) {
        retval_str = NAME_NAME;
    }

    if retval_str.is_empty() {
        return None;
    } else {
        return Some(String::from(retval_str));
    }
}

fn check_if_email(name: &str, data_type: &str) -> bool {
    name.to_lowercase().contains("email")
        || name.to_lowercase().contains("e-mail")
        || name.to_lowercase().contains("e_mail")
}

fn check_if_first_name(name: &str, data_type: &str) -> bool {
    name.to_lowercase().contains("first") && name.to_lowercase().contains("name")
}

fn check_if_last_name(name: &str, data_type: &str) -> bool {
    name.to_lowercase().contains("last") && name.to_lowercase().contains("name")
}

fn check_if_name(name: &str, data_type: &str) -> bool {
    name.to_lowercase().contains("name")
}

fn check_if_text(data_type: &str) {}
