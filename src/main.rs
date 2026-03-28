mod rusty_cache;
use rusty_cache::{Cache, CacheValues};
use std::{
    fs,
    io::{self, Write},
};

fn main() {
    let output_file = String::from("data/SAVE.txt");
    fs::create_dir_all("data").expect("Failed to create data directory");
    let mut cache = Cache::new(output_file);
    println!("RustyCache v0.1 (Startup Mode)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0].to_uppercase().as_str() {
            "INSERT" => {
                if parts.len() < 3 {
                    continue;
                }

                let key = parts[1].to_string();
                let value_raw = parts[2..].join(" ");

                let value = match value_raw.parse::<i32>() {
                    Ok(number) => CacheValues::Number(number),
                    Err(_) => CacheValues::Text(value_raw.to_string()),
                };

                cache.insert(key, value);
                println!("OK");
            }
            "GET" => {
                if parts.len() < 2 {
                    continue;
                }
                match cache.get(parts[1]) {
                    Some(val) => println!("{:?}", val),
                    None => println!("(nil)"),
                }
            }
            "DELETE" => {
                if parts.len() < 2 {
                    continue;
                }
                match cache.remove(parts[1]) {
                    Some(val) => println!("REMOVED: {:?}", val),
                    None => println!("NOTHING REMOVED"),
                }
            }
            "SAVE" => match cache.write_to_file() {
                Ok(_) => println!("Cache saved to file."),
                Err(e) => println!("Error saving cache: {}", e),
            },
            "EXIT" => break,
            _ => println!("Unknown command. Try INSERT, GET, DELETE, SAVE, or EXIT."),
        }
    }
}
