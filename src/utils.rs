use colored::Colorize;

pub fn hex_to_decimals(hex: &String, with_prefix: bool) -> i128 {
    let mut value = hex.as_str();
    if with_prefix == true {
        value = hex.trim_start_matches("0x");
    }
    return i128::from_str_radix(value, 16).unwrap();
}

pub fn format_label_and_value(label: &str, value: &String) -> std::string::String {
    format!(
        "{}: {}\n",
        format!("{}", label).bold().underline().white().on_green(),
        format!("{}", value).bold().underline()
    )
}