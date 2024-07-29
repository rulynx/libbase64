#![allow(
    dead_code,
    unused,
    unused_imports,
)]

extern crate walkdir;
extern crate std;

use ::std::path::{Path, PathBuf};
use ::std::collections::HashMap;
use ::walkdir::WalkDir;

fn find<P: AsRef<Path>>(root: P, file: &str) -> Option<PathBuf> {
    for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if e.file_name() == file {
            return Some(e.into_path());
        }
    }
    None
}

fn lib_root() -> Vec<PathBuf> {
    #[cfg(any(unix, target_os = "macos"))]
    let raw = vec!["/usr/lib", "/usr/local/lib"];
    #[cfg(windows)]
    let raw = vec!["C:\\Program Files (x86)\\Windows Kits\\10\\Lib", "C:\\Program Files\\Windows Kits\\10\\Lib", "C:\\Program Files (x86)\\Microsoft SDKs\\Windows\\v7.1A\\Lib"];
    let mut res: Vec<PathBuf> = Vec::new();
    for s in raw {
        let buf = PathBuf::from(s);
        if buf.exists() {
            res.push(buf);
        }
    }
    res
}

fn main() {

    let out = if ::std::env::var("OUT_DIR").is_ok() {
        ::std::env::var("OUT_DIR").unwrap()
    } else {
        ::std::env::temp_dir().to_str().unwrap().to_owned()
    };

    let out: &'static str = Box::leak(out.into_boxed_str());

    let mut found: bool = false;
    let mut count: usize = 0;

    for p in lib_root() {
        #[cfg(any(unix, target_os = "macos"))]
        {
            if find(p.clone(), "libb64.a").is_some() {
                found = true;
                count += 1;
                let path = find(p.clone(), "libb64.a").unwrap();
                // Laut neuster Rust build.rs api referenz von rust-lang soll man nicht mehr cargo:{attribut} verwenden sondern cargo::{attribut}.
                // Die variante mit : stat :: ist veraltet und rustc reagiert nicht mehr korrekt darauf.
                println!("cargo::rustc-link-search={}", path.to_str().unwrap());
                println!("cargo::rustc-link-lib=static=b64");
            }
            
            if find(p.clone(), "libb64.so").is_some() {
                found = true;
                count += 1;
                let path = find(p.clone(), "libb64.so").unwrap();
                println!("cargo::rustc-link-search={}", path.to_str().unwrap());
                println!("cargo::rustc-link-lib=dylib=b64");
            }

            if !found || count == 0 {
                panic!("Can't find libb64.a and/or libb64.so in {}", p.clone().display());
            }

            println!("cargo::rustc-link-lib=b64");
        }
        #[cfg(windows)]
        {
            if find(p.clone(), "libb64.lib").is_some() {
                found = true;
                count += 1;
                let path = find(p.clone(), "libb64.lib").unwrap();
                println!("cargo::rustc-link-search={}", path.to_str().unwrap());
                println!("cargo::rustc-link-lib=static=b64");
            }

            if find(p.clone(), "libb64.dll").is_some() {
                found = true;
                count += 1;
                let path = find(p.clone(), "libb64.dll").unwrap();
                println!("cargo::rustc-link-search={}", path.to_str().unwrap());
                println!("cargo::rustc-link-lib=dylib=b64");
            }

            if !found || count == 0 {
                panic!("Can't find libb64.lib and/or libb64.dll in {}", p.clone().display());
            }

            println!("cargo::rustc-link-lib=b64");
        }
    }
}