use std::path::Path;

pub fn is_country_code_valid(country_code: &str) -> bool {
    const FOLDER: &str = "config";
    const FILE_TYPE: &str = "config";
    let filepath = format!("{FOLDER}/{country_code}.{FILE_TYPE}");

    let file = Path::new(&filepath);

    file.exists()
}