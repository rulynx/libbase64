#![allow(
    unused_imports,
    unused,
    unused_lifetimes,
    incomplete_features,
    internal_features
)]

#![feature(
    const_size_of_val,
    const_trait_impl,
    effects,
)]

extern crate libc;
extern crate libbase64_sys;
extern crate std;
extern crate libbase64_derive;

#[cfg(
    any(
        all(
            feature = "encode",
            not(feature = "prelude"),
        ),
        all(
            feature = "encode",
            feature = "prelude"
        )
    )
)]
pub mod encode;

#[cfg(
    any(
        all(
            feature = "decode",
            not(feature = "prelude"),
        ),
        all(
            feature = "decode",
            feature = "prelude"
        )
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