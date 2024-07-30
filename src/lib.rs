#![feature(
    const_mut_refs
)]
extern crate libbase64_derive as b64derive;
extern crate libbase64_sys as b64sys;
extern crate std;
extern crate core;
extern crate libc;

pub mod encode;