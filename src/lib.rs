use std::process::Command;

pub fn exist(s: &str) -> bool {
    std::path::Path::new(s).exists()
}

pub fn exec(s: &str){
    std::path::Path::new(s).exists();
}