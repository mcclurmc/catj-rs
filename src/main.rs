use clap::{Arg, App};
use colored::*;
use json::{JsonValue, parse};
use std::io::{self, Read};
use std::fs::read_to_string;

fn print_json(json: &JsonValue, path: Option<&str>) {
    let sep = ".".cyan();

    match json {
        JsonValue::Null | JsonValue::Boolean(_) | JsonValue::Number(_) =>
            println!("{} = {}", path.unwrap_or(&sep), json),
        JsonValue::Short(_) | JsonValue::String(_) => {
            let s = format!("\"{}\"", json.as_str().unwrap()).green();
            println!("{} = {}", path.unwrap_or(&sep), s)
        }
        JsonValue::Array(_) => {
            for (i, x) in json.members().enumerate() {
                let path = format!("{}[{}]", path.unwrap_or(&sep), i.to_string().cyan());
                print_json(x, Some(&path))
            }
        },
        JsonValue::Object(_) => {
            for (e, x) in json.entries(){
                let path = format!("{}{}{}", path.unwrap_or_default(), &sep, e);
                print_json(x, Some(&path))
            }
        }
    };
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("catj-rs")
        .version("0.1")
        .arg(Arg::with_name("file")
            .help("JSON file to parse")
        )
        .get_matches();
    let file = matches.value_of("file");
    let file = match file {
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
        Some(file) => {
            read_to_string(file)?
        }
    };
    let file = parse(&file).unwrap();
    print_json(&file, None);

    Ok(())
}