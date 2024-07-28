//! This submodule provides Rust FFI bindings to the C header file `cencode.h`
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
//!
//! This is part of the libb64 project, and has been placed in the public domain.
//! For details, see [`lib64`](http://sourceforge.net/projects/libb64)
//! 
//! ```rust
//! use libbase64_sys::encode;
//! ```
#![allow(non_camel_case_types)]

/// # [`base64_encodestep`]
/// =======================
/// 
/// ## Metadata
/// 
/// |  **Meta Key**  |  **Meta Value**       |
/// |----------------|-----------------------|
/// |  **Name:**     | [`base64_encodestep`] |
/// |  **Type:**     | `enum`, repr(C)       |
/// |  **Since:**    | 0.1.0                 |
/// |  **Traits**    |                       |
/// |                |  **Derive**           |
/// |                | - [`Debug`]           |
/// |                | - [`Clone`]           |
/// |                | - [`Copy`]            |
/// |  **Category**  | safe                  |
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum base64_encodestep {
    /// # [`base64_encodestep::step_A`]
    /// 
    /// Enum member of [`base64_encodestep`]
    step_A,
    /// # [`base64_encodestep::step_B`]
    /// 
    /// Enum member of [`base64_encodestep`]
    step_B,
    /// # [`base64_encodestep::step_C`]
    /// 
    /// Enum member of [`base64_encodestep`]
    step_C,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct base64_encodestate {
    pub step: base64_encodestep,
    pub result: *mut libc::c_char,
    pub stepcount: libc::c_int,
}

#[cfg_attr(any(unix, target_os = "macos"), link(name = "b64"))]
#[cfg_attr(windows, link(name = "libb64"))]
extern "C" {
    pub fn base64_init_encodestate(state_in: *mut base64_encodestate);
    pub fn base64_encode_value(value_in: libc::c_char) -> libc::c_char;
    pub fn base64_encode_block(
        plaintext_in: *const libc::c_char,
        length_in: libc::c_int,
        code_out: *mut libc::c_char,
        state_in: *mut base64_encodestate
    ) -> libc::c_int;
    pub fn base64_encode_blockend(code_out: *mut libc::c_char, state_in: *mut base64_encodestate) -> libc::c_int;
}