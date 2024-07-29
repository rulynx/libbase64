#![allow(
    unused,
    unused_imports
)]

extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse, parse2, parse_macro_input, DeriveInput};
use quote::quote;