use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn load_config(path: &str) -> Result<HashMap<String, String>, io::Error> {
    
    let mut configs = HashMap::new();

    let file = File::open(path)?;

    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split('=').collect(); // ["key", "value"]

        if parts.len() != 2 {
            continue;
        }

        configs.entry(parts[0].to_string()).or_insert(parts[1].to_string());
    }

    Ok(configs)
}


fn main() {
    let configs = load_config("D:\\Desktop\\Formations\\Rust\\rust-book-projects\\exercises\\config_loader\\config.txt");

    match configs {
        Ok(map) => {
            for (config, value) in map {
                println!("{}: {}", config, value);
            }
        },
        Err(error) => eprintln!("Failed to read the configurations: {error:?}"),
    }
}
