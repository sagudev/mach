use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
mod v0;

fn main() {
    let mut file = File::open("mach.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    //println!("readed: {}", contents);
    let json: Value = serde_json::from_str(&contents).unwrap();
    let json = json.as_object().unwrap();
    let version = json.get("version").unwrap().as_u64().unwrap();
    println!("version: {:?}", version);
}
