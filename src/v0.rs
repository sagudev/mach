use std::collections::HashMap;
use std::fmt::Debug;

/// Command
struct COM {
    /// 0:sudo apt update
    /// 1:sudo make install
    exec: Vec<String>,
    /// hlp
    help: String,
}

fn com(exec: &str, help: &str) -> COM {
    COM {
        exec: [exec.to_string()].to_vec(),
        help: help.to_string(),
    }
}

impl Default for COM {
    fn default() -> Self {
        Self {
            exec: ["".to_string()].to_vec(),
            help: format!("Empty"),
        }
    }
}

pub enum PRE {
    Rust,
    C,
    Mozilla,
    None,
}

impl From<&str> for PRE {
    fn from(x: &str) -> Self {
        match x.to_lowercase().as_str() {
            "rust" => PRE::Rust,
            "c" => PRE::C,
            "moz" => PRE::Mozilla,
            "mozilla" => PRE::Mozilla,
            _ => PRE::None,
        }
    }
}

/// (shell, /bin/bash)
struct ENV(String, String);

impl ENV {
    fn set(&self) {
        std::env::set_var(&self.0, &self.1);
    }
}

struct Mach {
    name: String,
    help: String,
    env: Option<Vec<ENV>>,
    commands: Option<HashMap<String, COM>>,
    bootstrap: COM,
    build: COM,
    run: COM,
    clean: COM,
}

impl Default for Mach {
    fn default() -> Self {
        let pwd = std::env::current_dir().unwrap();
        let name: String = pwd
            .strip_prefix(pwd.parent().unwrap())
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Self {
            name: name.clone(),
            help: format!("Autogen mach wrapper for {}", name),
            ..Default::default()
        }
    }
}

impl Mach {
    fn pre(pre: PRE) -> Self {
        match pre {
            PRE::C => Self {
                build: com("make", "Build using make"),
                bootstrap: com("apt install build-essential", "Install dependencies"),
                clean: com("make clean", "Clean tree"),
                commands: Some({
                    let mut t = HashMap::new();
                    t.insert("install".to_string(), com("sudo make install", "Install"));
                    t
                }),
                ..Default::default()
            },
            PRE::Rust => Self {
                build: com("cargo build", "Compile the project"),
                bootstrap: com(
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
                    "Install rust via rustup",
                ),
                clean: com("cargo clean", "Remove the target directory"),
                run: com("cargo run", "Run a Rust binary."),
                commands: Some({
                    let mut t = HashMap::new();
                    t.insert("fmt".to_string(), com("cargo fmt", "Run a Rust binary."));
                    t
                }),
                ..Default::default()
            },
            PRE::Mozilla => Self {
                build: com("./mach build", "Compile firefox"),
                bootstrap: com("./mach bootstrap", "Install dependencies"),
                clean: com("./mach clobber", "Clobber tree"),
                run: com("./mach run", "Run build product."),
                ..Default::default()
            },
            PRE::None => Self {
                ..Default::default()
            },
        }
    }
}

pub fn mach(data: &yaml_rust::yaml::Yaml) {
    println!("{:#?}", data);
    let mut mach: Mach = if let Some(pre) = data["pre"].as_str() {
        Mach::pre(pre.into())
    } else {
        Mach::pre(PRE::None)
    };
    if let Some(name) = data["name"].as_str() {
        mach.name = name.to_string();
    }
    if let Some(help) = data["hlp"].as_str() {
        mach.help = help.to_string();
    }
    if let Some(run) = data["run"].as_str() {
        mach.run = run.to_string();
    }
    //for el in data.as_hash().unwrap() {
    //    println!("loo");
    //    println!("{:#?}", el);
    //}
}
