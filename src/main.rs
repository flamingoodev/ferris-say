use std::env;

#[macro_use]
extern crate serde_derive;

use ferris_says::say;
use rand::Rng;
use serde_json;
use std::fmt::Error;
use std::fs::File;
use std::io::{stdout, BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    content: String,
}

fn main() {
    let args: Vec<String> = env::args().filter(|s| !s.contains("ferris-say")).collect();
    let mut input = String::new();
    let content = get_from_json();
    match content {
        None => {
            for arg in args {
                input = input + &*arg + &*String::from(" ");
            }
        }
        Some(item) => {
            input = item
        }
    }
    let out = b"Hello Rustaceans!";
    let width = 24;
    let mut writer = BufWriter::new(stdout());
    if input.len() > 0 {
        say(input.as_bytes(), width, &mut writer).unwrap();
    } else {
        say(out, width, &mut writer).unwrap();
    }
}

fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Content>, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path).expect("File open error");
    let reader = BufReader::new(file);
    // Read the JSON contents of the file as an instance of `Vec<Content>`.
    let u = serde_json::from_reader(reader).expect("Json error");
    // Return the `Vec<Content>`.
    Ok(u)
}

fn get_from_json() -> Option<String> {
    let content = read_from_file("src/resources/content.json");
    let vec_content = content.expect("JSON ERROR");
    let rand_index = rand::thread_rng().gen_range(0..(vec_content.len() - 1));
    let out = vec_content.get(rand_index);
    match out {
        None => {
            None
        }
        Some(item) => {
            Some(item.content.clone())
        }
    }
}
