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
#![allow(non_camel_case_types)]

/// # [`base64_decodestep`]
/// =======================
/// 
/// ## Metadata
/// 
/// |  **Meta Key**  |  **Meta Value**       |
/// |----------------|-----------------------|
/// |  **Name:**     | [`base64_decodestep`] |
/// |  **Type:**     | `enum`, repr(C)       |
/// |  **Since:**    | 0.1.0                 |
/// |  **Traits:**   |                       |
/// |                |  **Derive**           |
/// |                | - [`Debug`]           |
/// |                | - [`Clone`]           |
/// |                | - [`Copy`]            |
/// |  **Category:** | safe                  |
#[doc(
    alias = "decodestep",
    alias = "step"
)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum base64_decodestep {
    /// - [`base64_decodestep::step_a`]
    /// 
    /// - Enum member of [`base64_decodestep`].
    step_a,
    /// - [`base64_decodestep::step_b`]
    /// 
    /// - Enum member of [`base64_decodestep`].
    step_b,
    /// - [`base64_decodestep::step_c`]
    /// 
    /// - Enum member of [`base64_decodestep`].
    step_c,
    /// - [`base64_decodestep::step_d`]
    /// 
    /// - Enum member of [`base64_decodestep`].
    step_d,
}

/// # [`base64_decodestep`]
/// =======================
/// 
/// ## Metadata
/// 
/// |  **Meta Key**  |  **Meta Value**       |
/// |----------------|-----------------------|
/// |  **Name:**     | [`base64_decodestate`]|
/// |  **Type:**     | `struct`, repr(C)     |
/// |  **Since:**    | 0.1.0                 |
/// |  **Traits:**   |                       |
/// |                |  **Derive**           |
/// |                | - [`Debug`]           |
/// |                | - [`Clone`]           |
/// |                | - [`Copy`]            |
/// |  **Category:** | safe                  |
#[doc(
    alias = "decodestate",
    alias = "state"
)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct base64_decodestate {
    /// - [`base64_decodestate::step`].
    /// 
    /// - **Type**: [`base64_decodestep`].
    pub step: base64_decodestep,
    /// - [`base64_decodestate::plainchar`].
    /// 
    /// - **Type**: [`libc::c_char`].
    pub plainchar: libc::c_char,
}

#[cfg_attr(any(unix, target_os = "macos"), link(name = "b64"))]
#[cfg_attr(windows, link(name = "libb64"))]
extern "C" {

    /// # [`base64_init_decodestate`]
    /// =======================
    /// 
    /// ## Metadata
    /// 
    /// |  **Meta Key**  |  **Meta Value**             |
    /// |----------------|-----------------------------|
    /// |  **Name:**     | [`base64_init_decodestate`] |
    /// |  **Type:**     | `function`                  |
    /// |  **Since:**    | 0.1.0                       |
    /// |  **Category:** | unsafe, extern, C           |
    #[doc(
    alias = "init",
    alias = "base64_init",
    alias = "b64_init",
    alias = "init_function"
    )]
    pub fn base64_init_decodestate(state_in: *mut base64_decodestate);

    /// # [`base64_decode_value`]
    /// =======================
    /// 
    /// ## Metadata
    /// 
    /// |  **Meta Key**  |  **Meta Value**             |
    /// |----------------|-----------------------------|
    /// |  **Name:**     | [`base64_decode_value`]     |
    /// |  **Type:**     | `function`                  |
    /// |  **Since:**    | 0.1.0                       |
    /// |  **Category:** | unsafe, extern, C           |
    #[doc(
        alias = "encode",
        alias = "encode_value",
        alias = "base64_encode",
        alias = "b64_encode",
        alias = "encode_function"
    )]
    pub fn base64_decode_value(value_in: libc::c_char) -> libc::c_int;

    /// # [`base64_decode_block`]
    /// =======================
    /// 
    /// ## Metadata
    /// 
    /// |  **Meta Key**  |  **Meta Value**             |
    /// |----------------|-----------------------------|
    /// |  **Name:**     | [`base64_decode_block`]     |
    /// |  **Type:**     | `function`                  |
    /// |  **Since:**    | 0.1.0                       |
    /// |  **Category:** | unsafe, extern, C           |
    #[doc(
        alias = "encode",
        alias = "encode_block",
        alias = "base64_encode",
        alias = "b64_encode",
        alias = "encode_function"
    )]
    pub fn base64_decode_block(
        code_in: *const libc::c_char,
        length_in: *const libc::c_int,
        plaintext_out: *mut libc::c_char,
        state_in: *mut base64_decodestate
    ) -> libc::c_int;
}