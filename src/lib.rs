#![allow(
    unused,
    unused_imports
)]

#![feature(
    deref_pure_trait
)]

extern crate libbase64_derive as b64derive;
extern crate libbase64_sys as b64sys;
extern crate libc;
extern crate std;
extern crate core;

pub(crate) mod error;