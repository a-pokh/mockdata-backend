use crate::constants::*;
use regex::Regex;

#[derive(Debug)]
pub struct FakeDataType {
    name: String,
    category_name: String,
}

#[derive(Debug)]
struct TextType {
    length: Option<u8>,
    varying: bool,
}
#[derive(Debug)]
struct NumericType {
    is_floating: bool,
}

pub fn get_data_type(
    name: &str,
    data_type: &str,
    is_primary_key: bool,
    is_table_has_composite_pk: bool,
    is_reference: bool,
    is_enum: bool,
    is_unique: bool,
) -> Option<String> {
    // enum
    if is_enum {
        return Some(String::from(data_type));
    }

    // money
    if data_type.to_lowercase().contains("money") {
        return Some(String::from(MONEY_MONEY));
    }

    // network
    if data_type.to_lowercase().contains("cidr") || data_type.to_lowercase().contains("inet") {
        return Some(String::from(NETWORK_IPV4));
    }
    if data_type.to_lowercase().contains("macaddr") {
        return Some(String::from(NETWORK_MAC));
    }

    // boolean
    if data_type.to_lowercase().contains("boolean") {
        return Some(String::from(BOOLEAN_BOOLEAN));
    }

    let text_type = get_text_type(data_type);

    // id
    if is_primary_key && !is_table_has_composite_pk {
        match &text_type {
            Some(text_type) => {
                match text_type.length {
                    Some(length) => {
                        if length < 20 {
                            return Some(String::from(ID_SHORT_UUID));
                        } else if length < 30 {
                            return Some(String::from(ID_CUID));
                        } else {
                            return Some(String::from(ID_CUID));
                        }
                    }
                    None => {
                        return Some(String::from(ID_CUID));
                    }
                };
            }
            _ => {}
        }

        if data_type.to_lowercase().contains("uuid") {
            return Some(String::from(ID_UUID));
        } else {
            return Some(String::from(ID_AUTOINCREMENT));
        }
    }

    // text
    if *&text_type.is_some() {
        if check_if_email(name, data_type) {
            return Some(String::from(INTERNET_EMAIL));
        } else if check_if_first_name(name, data_type) {
            return Some(String::from(NAME_FIRST_NAME));
        } else if check_if_last_name(name, data_type) {
            return Some(String::from(NAME_LAST_NAME));
        } else if check_if_name(name, data_type) {
            return Some(String::from(NAME_NAME));
        } else {
            return Some(String::from(LOREM_WORD));
        }
    }
    // number
    match get_numeric_type(data_type) {
        Some(numeric_type) => {
            if numeric_type.is_floating {
                return Some(String::from(NUMBER_FLOAT));
            } else {
                return Some(String::from(NUMBER_INTEGER));
            }
        }
        _ => {}
    }
    // date
    if let Some(date_type) = get_date_type(data_type) {
        return Some(String::from(date_type));
    }

    None
}

fn check_if_id(is_primary_key: bool) -> bool {
    return is_primary_key;
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

fn get_date_type(data_type: &str) -> Option<&str> {
    if data_type.to_lowercase().contains("timestamp")
        && data_type.to_lowercase().contains("without time zone")
    {
        return Some(DATE_TIMESTAMP);
    } else if (data_type.to_lowercase().contains("timestamp")
        && data_type.to_lowercase().contains("with time zone"))
        || data_type.to_lowercase().contains("timestamptz")
    {
        return Some(DATE_TIMESTAMP_WITH_TIMEZONE);
    } else if data_type.to_lowercase().contains("timestamp") {
        return Some(DATE_TIMESTAMP);
    }
    if data_type.to_lowercase().contains("time")
        && data_type.to_lowercase().contains("without time zone")
    {
        return Some(DATE_TIME);
    } else if (data_type.to_lowercase().contains("time")
        && data_type.to_lowercase().contains("with time zone"))
        || data_type.to_lowercase().contains("timetz")
    {
        return Some(DATE_TIME_WITH_TIMEZONE);
    } else if data_type.to_lowercase().contains("time") {
        return Some(DATE_TIME);
    }
    if data_type.to_lowercase() == "date" {
        return Some(DATE_DATE);
    }

    None
}

fn get_text_type(data_type: &str) -> Option<TextType> {
    // compile outside loop
    let varchar_regex = Regex::new(r"(?i)varchar\((\d+)\)|character varying\((\d+)\)").unwrap();
    let char_regex = Regex::new(r"(?i)char\((\d+)\)|character\((\d+)\)").unwrap();

    let varchar_match = varchar_regex.captures(data_type);
    let char_match = char_regex.captures(data_type);
    if data_type.to_lowercase() == "text"
        || data_type.to_lowercase() == "citext"
        || data_type.to_lowercase() == "varchar"
        || data_type.to_lowercase() == "character varying"
    {
        return Some(TextType {
            length: None,
            varying: true,
        });
    }
    match varchar_match {
        Some(varchar_capture) => {
            let mut length: Option<u8> = None;
            if let Some(i) = varchar_capture.get(1) {
                length = Some(i.as_str().parse().unwrap());
            }
            if let Some(i) = varchar_capture.get(2) {
                length = Some(i.as_str().parse().unwrap());
            }
            return Some(TextType {
                length,
                varying: true,
            });
        }
        _ => {}
    }
    match char_match {
        Some(char_capture) => {
            let mut length: Option<u8> = None;
            if let Some(i) = char_capture.get(1) {
                length = Some(i.as_str().parse().unwrap());
            }
            if let Some(i) = char_capture.get(2) {
                length = Some(i.as_str().parse().unwrap());
            }
            return Some(TextType {
                length,
                varying: false,
            });
        }
        _ => {}
    }

    None
}

// TODO: numeric(n,0) is not float!!
fn get_numeric_type(data_type: &str) -> Option<NumericType> {
    let int_types_vector = vec![
        "smallint", "int2", "integer", "int", "int4", "int8", "bigint",
    ];
    let float_types_vector = vec!["real", "float4", "double precision", "float8", "numeric"];

    if int_types_vector
        .iter()
        .any(|&i| i == data_type.to_lowercase())
    {
        return Some(NumericType { is_floating: false });
    }

    let float_var_regex =
        Regex::new(r"(?i)decimal\((\d+, ?\d+)\)|numeric\((\d+,?\d+)\)|numeric").unwrap();
    if float_types_vector
        .iter()
        .any(|&i| i == data_type.to_lowercase())
        || float_var_regex.is_match(data_type)
    {
        return Some(NumericType { is_floating: true });
    }

    None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_text_returns_varying_char_with_length() {
        let result = get_text_type("varchar(20)").unwrap();

        assert_eq!(result.length.unwrap(), 20);
        assert_eq!(result.varying, true);
    }
    #[test]
    fn get_text_returns_char_with_length() {
        let result = get_text_type("char(10)").unwrap();

        assert_eq!(result.length.unwrap(), 10);
        assert_eq!(result.varying, false);
    }
    #[test]
    fn get_text_returns_text() {
        let result = get_text_type("text").unwrap();

        assert_eq!(result.length, None);
        assert_eq!(result.varying, true);
    }
    #[test]
    fn get_text_returns_none() {
        let result = get_text_type("numeric");

        assert!(result.is_none());
    }
    #[test]
    fn get_numeric_returns_number_integer() {
        let result = get_numeric_type("smallint").unwrap();

        assert_eq!(result.is_floating, false);
    }
    #[test]
    fn get_numeric_returns_number_float() {
        let result = get_numeric_type("real").unwrap();

        assert_eq!(result.is_floating, true);
    }

    #[test]
    fn get_numeric_returns_number_float_var() {
        let result = get_numeric_type("numeric(2, 2)").unwrap();

        assert_eq!(result.is_floating, true);
    }
    #[test]
    fn get_numeric_returns_none() {
        let result = get_numeric_type("text");

        assert!(result.is_none());
    }
}
