use std::fs::File;
use std::io::Read;
use yaml_rust::YamlLoader;
mod lib;
use lib::exist;
mod v0;

/// If mach.yml else mach(.sh) else detect
fn main() {
    //let path = std::env::current_dir().unwrap();
    //println!("The current directory is {}", path.display());
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
            0 => {
                v0::mach(docs);
            }
            _ => {
                panic!("Wrong version");
            }
        }
    } else if exist("mach") {
        // check if it is mozilla tree
        if exist("moz.build") {
            pass();
        } else {
            pass();
        }
    } else if exist("mach.sh") {
        // send to sh
        pass();
    } else {
        // detect
        detect();
    }
}

fn pass() {
    todo!("detection is not impl.");
    /* if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    }; */
}

fn detect() {
    todo!("detection is not impl.");
}
