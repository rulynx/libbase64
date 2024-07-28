//! This mainmodule provides Rust FFI bindings to the C header files `cdecode.h`
//! from `libbase64`.
//!
//! The goal is to fully bind the C file to Rust, allowing us to use it in Rust
//! just as we would in C. These are raw bindings and will likely require
//! `unsafe` calls and possibly the `libc` crate to work with the data types.
//!
//! If you don't have `libc` available or prefer not to use it, you can find the
//! correct type declarations in both `std::os::raw` and `core::ffi`.
//! 
//! `cdecode.h` - c header for a base64 decode algorithm
//!
//! This is part of the libb64 project, and has been placed in the public domain.
//! For details, see [`lib64`](http://sourceforge.net/projects/libb64)
//! 
//! ```rust
//! use libbase64_sys::decode;
//! ```