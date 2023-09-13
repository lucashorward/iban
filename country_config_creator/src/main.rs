use std::fs;
use std::env;
use std::path::Path;

fn prepare_location(path: &Path) {
    let path_pretty = path.display();
    if path.exists() {
        println!("Throwing away {path_pretty}");
        let _ = fs::remove_dir_all(path);
    }
    println!("Creating {path_pretty}");
    let _ = fs::create_dir(path);
}

fn create_readme(path: &str) {
    let file_path = format!("{path}/README.md");
    let contents = "NOTE: This folder is removed every time the country_config_creator is run. Do not store anything here. These files are configs per country.";

    let _ = fs::write(file_path, contents);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args[3].is_empty() {
        panic!("No path provided!");
    }
    let output_path = &args[3];
    const OUTPUT_FOLDER: &str = "config/";
    let output_location = format!("{output_path}/{OUTPUT_FOLDER}");
    let base_path = Path::new(&output_location);
    prepare_location(base_path);
    create_readme(&output_location);

    let current = env::current_dir().unwrap();
    let display = current.display();
    const FILE_PATH: &str = "country_config_creator/config/countries.config";
    let config_location = format!("{display}/{FILE_PATH}");

    let countries_data = fs::read_to_string(&config_location).unwrap();

    let country_list: Vec<&str> = countries_data.split('\n').collect();

    for country in country_list {
        let file_path = format!("{output_location}/{country}.config");
        let contents = format!("country_code={country}");

        let _ = fs::write(file_path, contents);
    }
}
