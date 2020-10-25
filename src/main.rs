use std::fs::File;
use std::io::Read;
use yaml_rust::YamlLoader;
mod v0;

/// If mach.yml else mach(.sh) else detect
fn main() {
    // if mach file open
    if let Ok(mut file) = File::open("mach.yml") {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let docs = &YamlLoader::load_from_str(&contents).unwrap()[0];
        let version = if !docs["version"].is_badvalue() {
            docs["version"].as_i64().unwrap()
        } else if !docs["ver"].is_badvalue() {
            docs["ver"].as_i64().unwrap()
        } else {
            panic!("No ver!!!");
        };
        println!("Detected version: {}", version);
        match version {
            0 => { v0::mach(docs); }
            _ => {
                panic!("Wrong version");
            }
        }
    } else if true {// else if mach.sh
        // send to sh
    } else {
        // detect
    }
}
