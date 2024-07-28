#![allow(
    unused,
    unused_imports,
)]

use std::path::PathBuf;
use std::process::Command;

fn main() {
    let include_dirs = vec!["/usr/include", "/usr/local/include"];
    let mut lib_dirs = vec!["/usr/lib", "/usr/local/lib"];

    // Füge system-spezifische Pfade hinzu
    if cfg!(target_family = "unix") {
        let host_triple = get_host_triple();
        lib_dirs.push(Box::leak(format!("/usr/lib/{}", host_triple).into_boxed_str()));
        lib_dirs.push(Box::leak(format!("/usr/local/lib/{}", host_triple).into_boxed_str()));
    } else if cfg!(target_os = "windows") {
        lib_dirs.push("C:\\Program Files (x86)\\Windows Kits\\10\\Lib");
        lib_dirs.push("C:\\Program Files\\Windows Kits\\10\\Lib");
        lib_dirs.push("C:\\Program Files (x86)\\Microsoft SDKs\\Windows\\v7.1A\\Lib");
    } else {
        panic!("Unsupported OS");
    }

    for dir in include_dirs {
        let path = PathBuf::from(dir);
        if path.exists() {
            println!("cargo::rustc-link-search={}", path.display());
        } else {
            eprintln!("No include dir found on device: {}", path.display());
            continue;
        }

        let b64_path = path.join("b64");
        if b64_path.exists() {
            println!("cargo::rustc-link-search={}", b64_path.display());
        } else {
            eprintln!("No b64 found on device: {}", b64_path.display());
            continue;
        }

        let file1 = b64_path.join("cencode.h");
        let file2 = b64_path.join("cdecode.h");
        if !file1.exists() {
            eprintln!("Can find 'b64' on device, but not: {}", file1.display());
            continue;
        } else if !file2.exists() {
            eprintln!("Can find 'b64' on device, but not: {}", file2.display());
            continue;
        }
    }

    for dir in lib_dirs {
        let path = PathBuf::from(dir);
        let triple_path = path.join(get_host_triple());
        if triple_path.exists() {
            println!("cargo::rustc-link-search={}", triple_path.display());
        } else {
            eprintln!("No lib dir found on device: {}", triple_path.display());
            continue;
        }

        #[cfg(any(unix, target_os = "macos"))]
        {
            println!("cargo::rustc-link-lib=static=b64");
            println!("cargo::rustc-link-lib=dylib=b64");
        }

        #[cfg(target_os = "windows")]
        {
            println!("cargo::rustc-link-lib=b64");
        }
    }
}

fn get_host_triple() -> String {
    let output = Command::new("rustc")
        .arg("--version")
        .arg("--verbose")
        .output()
        .expect("Failed to execute rustc --version --verbose");
    let output_str = String::from_utf8(output.stdout).expect("Invalid UTF-8 in rustc output");
    for line in output_str.lines() {
        if line.starts_with("host:") {
            return line.split_whitespace().nth(1).unwrap().to_string();
        }
    }
    panic!("Failed to determine host triple");
}