#[allow(dead_code)]

// use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::fs::DirEntry;
use std::ffi::OsStr;
use std::io;

fn main() {

    let work_dir: &str = "E:\\TestFolder\\";
    let dir: &Path = Path::new(work_dir);

    if dir.is_dir() {
        let extensions: Vec<String> = collect_extensions(&dir);

        for extension in extensions.iter() {
            println!("final extension: {}", extension);
        }
    }
}


fn collect_extensions(dir: &Path) -> Vec<String> {
    let mut exts: Vec<String> = Vec::new();

    for item in fs::read_dir(dir).unwrap() {
        let entry = item.unwrap();
        let path = entry.path();
        let ext = path.extension();

        if let Some(os_str) = ext {
            if let Some(ext_str) = os_str.to_str() {
                exts.push(String::from(ext_str));
            }
        }
    }

    exts
}