#![allow(
    unused,
    unused_imports,
)]

use std::path::PathBuf;
use std::process::Command;

fn main() {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        println!("cargo::rustc-link-search=/usr/lib/x86_64-linux-gnu");
        println!("cargo::rustc-link-search=/usr/local/lib/x86_64-linux-gnu");
        println!("cargo::rustc-link-lib=static=b64");
        println!("cargo::rustc-link-lib=dylib=b64");
        println!("cargo::rustc-link-lib=b64");
    }
    #[cfg(all(target_os = "linux", target_arch = "x86"))]
    {
        println!("cargo::rustc-link-search=/usr/lib/i386-linux-gnu");
        println!("cargo::rustc-link-search=/usr/local/lib/i386-linux-gnu");
        println!("cargo::rustc-link-lib=static=b64");
        println!("cargo::rustc-link-lib=dylib=b64");
        println!("cargo::rustc-link-lib=b64");
    }
}