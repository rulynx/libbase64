#![allow(
    unused,
    unused_imports
)]
#![feature(
    proc_macro_expand
)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse, parse2, parse_str, DeriveInput, Data, Generics, TypeParamBound, TraitBound, TraitBoundModifier, Path, GenericParam};
use quote::quote;

#[proc_macro_derive(Send)]
pub fn derive_send(token: TokenStream) -> TokenStream {
    let input = parse::<DeriveInput>(token).unwrap();
    let name = &input.ident;
    let gene = bound_trait(input.generics, "Send");
    let (impl_gene, ty_gene, where_clause) = gene.split_for_impl();
    if where_clause.is_some() {
        return TokenStream::from(quote! {
            unsafe impl<#impl_gene> core::marker::Send for #name<#ty_gene> #where_clause.unwrap() {}
        });
    } else {
        return TokenStream::from(quote! {
            unsafe impl<#impl_gene> core::marker::Send for #name<#ty_gene> {}
        });
    }
}

#[proc_macro_derive(Sync)]
pub fn derive_sync(token: TokenStream) -> TokenStream {
    if parse::<DeriveInput>(token.clone()).is_err() {
        match parse::<DeriveInput>(token) {
            Ok(_) => todo!(),
            Err(e) => return TokenStream::from(e.into_compile_error()),
        }
    }

    let input = parse::<DeriveInput>(token).unwrap();
    let name = &input.ident;
    let gene = bound_trait(input.generics, "Sync");
    let (impl_gene, ty_gene, where_clause) = gene.split_for_impl();

    if where_clause.is_some() {
        return TokenStream::from(quote! {
            unsafe impl<#impl_gene> core::marker::Sync for #name<#ty_gene> #where_clause.unwrap() {}
        });
    } else {
        return TokenStream::from(quote! {
            unsafe impl<#impl_gene> core::marker::Sync for #name<#ty_gene> {}
        });
    }
}

fn bound_trait(mut gene: Generics, name: &'static str) -> Generics {
    let trait_path: Path = parse_str(name).expect(&format!("can't find path for trait {}", name));

    let trait_bound = TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: trait_path,
    });

    for param in &mut gene.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            if !type_param.bounds.iter().any(|b| matches!(b, TypeParamBound::Trait(tb) if tb.path.is_ident(name))) {
                type_param.bounds.push(trait_bound.clone());
            }
        }
    }

    gene
}