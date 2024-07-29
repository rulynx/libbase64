# libbase64-sys

An FFI Binding to libb64.

## Meta

| **Meta Key**       | **Meta Value**                        |
|--------------------|---------------------------------------|
| **Name:**          | *libbase64-sys*                       |
| **Version:**       | *0.1.0-alpha.0*                       |
| **Depends**        | *libc*                                |
| **License:**       | *Public Domain*                       |
| **Bound:**         | *cencode.h*, *cdecode.h*              |
| **Repository:**    | *https://github.com/rulynx/libbase64* |
| **Documentation**  | *https://docs.rs/libbase64-sys*       |

## Adding

Run `cargo add libbase64-sys` for adding libbase64-sys to your depends.

## Warning

<div class="warning">

**Attention**

This crate is a *raw binding* crate. If you have never worked with **C** or **raw pointers** before, you should not use this crate.

There will be a [second crate](https://docs.rs/libbase64) that builds a safe wrapper around the *raw bindings*.

This crate is well-suited as a basis for your own implementation.

</div>

