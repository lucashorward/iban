use std::fs;
use std::env;
use std::path::Path;

fn prepare_location(path: &str) {
    let base_path = Path::new(path);
    let path_pretty = base_path.display();
    if base_path.exists() {
        println!("Throwing away {path_pretty}");
        let _ = fs::remove_dir_all(base_path);
    }
    println!("Creating {path_pretty}");
    let _ = fs::create_dir(base_path);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 || args[3].is_empty() {
        panic!("No path provided!");
    }
    let output_path = &args[3];
    const OUTPUT_FOLDER: &str = "config/";
    let output_location = format!("{output_path}/{OUTPUT_FOLDER}");
    prepare_location(&output_location);

    let current = env::current_dir().unwrap();
    let display = current.display();
    const FILE_PATH: &str = "country_config_creator/config/countries.config";
    let config_location = format!("{display}/{FILE_PATH}");

    let countries_data = fs::read_to_string(&config_location).unwrap();

    let country_list: Vec<&str> = countries_data.split("\n").collect();

    for country in country_list {
        let file_path = format!("{output_location}/{country}.config");
        let contents = format!("country_code={country}");

        let _ = fs::write(file_path, contents);
    }

    println!("Hello, world!, {config_location}");
}
