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
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[doc(alias = "Send_Derive")]
#[proc_macro_derive(Send)]
pub fn derive_send(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_block, type_block, where_clause) = generics.split_for_impl();
    if where_clause.is_some() {
        TokenStream::from(quote! {
            unsafe impl #impl_block ::core::marker::Send for #name #type_block #where_clause.unwrap() {}
        })
    } else {
        TokenStream::from(quote! {
            unsafe impl #impl_block ::core::marker::Send for #name #type_block {}
        })
    }
}

#[doc(alias = "Sync_Derive")]
#[proc_macro_derive(Sync)]
pub fn derive_sync(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_block, type_block, where_clause) = generics.split_for_impl();
    if where_clause.is_some() {
        TokenStream::from(quote! {
            unsafe impl #impl_block ::core::marker::Sync for #name #type_block #where_clause.unwrap() {}
        })
    } else {
        TokenStream::from(quote! {
            unsafe impl #impl_block ::core::marker::Sync for #name #type_block {}
        })
    }
}