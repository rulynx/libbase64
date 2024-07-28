//! This mainmodule provides Rust FFI bindings to the C header files `cencode.h` and `cdecode.h`
//! from `libbase64`.
//!
//! The goal is to fully bind the C file to Rust, allowing us to use it in Rust
//! just as we would in C. These are raw bindings and will likely require
//! `unsafe` calls and possibly the `libc` crate to work with the data types.
//!
//! If you don't have `libc` available or prefer not to use it, you can find the
//! correct type declarations in both `std::os::raw` and `core::ffi`.
//! 
//! `cencode.h` - c header for a base64 encoding algorithm
//! `cdecode.h` - c header for a base64 decode algorithm
//!
//! This is part of the libb64 project, and has been placed in the public domain.
//! For details, see [`lib64`](http://sourceforge.net/projects/libb64)
//! 
//! ```rust
//! use libbase64_sys
//! ```
#![warn(missing_docs)]
#![deny(missing_docs)]
#![feature(decl_macro)]
#![allow(
    unused,
    unused_imports,
    unknown_lints,
)]

extern crate libc;

#[cfg(
    all(
        feature = "encode",
        not(feature = "prelude")
    )
)]
pub mod encode;

#[cfg(
    all(
        feature = "decode",
        not(feature = "prelude")
    )
)]
pub mod decode;

#[cfg(
    all(
        feature = "prelude",
        not(feature = "encode")
    )
)]
pub(crate) mod encode;

#[cfg(
    all(
        feature = "prelude",
        not(feature = "decode")
    )
)]
pub(crate) mod decode;

#[cfg(feature = "prelude")]
pub mod prelude {
    //! # [`crate::prelude`]
    //! 
    //! This Sub modul imports all Functions, Structs, Enums and co
    //! In a Prelude, which you can import easier.
    //! 
    //! # Example
    //! 
    //! ```rust
    //! use libbase64_sys::prelude;
    //! ```
    pub use crate::encode::*;
    pub use crate::decode::*;
}

#[cfg(feature = "prelude_root")]
pub use crate::prelude::*;