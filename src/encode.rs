//!
#![allow(
    unused,
    unused_imports
)]
#![deny(missing_docs)]
#![warn(missing_docs)]

use super::libc;
use super::std;
use super::core;
use super::b64derive::{Send, Sync};
use super::b64sys::{base64_encode_block, base64_encode_blockend, base64_encode_value, base64_encodestate, base64_encodestep, base64_init_encodestate};

///
#[derive(Debug, Clone, Send, Sync)]
pub struct EncodeState {
    pub(crate) state: base64_encodestate,
}

impl From<base64_encodestate> for EncodeState {
    fn from(value: base64_encodestate) -> Self {
        EncodeState { state: value }
    }
}

impl Default for EncodeState {
    fn default() -> Self {
        Self::new()
    }
}

impl EncodeState {

    ///
    #[inline]
    pub fn new() -> EncodeState {
        let mut state_uninit = base64_encodestate { step: base64_encodestep::step_A, stepcount: 0, result: core::ptr::null_mut() };
        unsafe { base64_init_encodestate(&mut state_uninit) };
        EncodeState { state: state_uninit }
    }
}

///
#[derive(Debug, Clone, Send, Sync)]
pub struct StateParser {

    pub(crate) state: EncodeState,
    pub(crate) parsed: Option<Vec<u8>>,
    pub(crate) new: bool,

}

impl From<EncodeState> for StateParser {
    fn from(value: EncodeState) -> Self {
        StateParser { state: value, parsed: None, new: true }
    }
}

impl From<base64_encodestate> for StateParser {
    fn from(value: base64_encodestate) -> Self {
        let state = EncodeState::from(value);
        StateParser { state, parsed: None, new: true }
    }
}

impl Default for StateParser {
    fn default() -> Self {
        Self::new()
    }
}

impl StateParser {
    
    ///
    #[inline]
    pub fn new() -> StateParser {
        let state = EncodeState::new();
        StateParser { state, parsed: None, new: true }
    }

    ///
    #[inline]
    pub const fn raw_parse(&self) -> base64_encodestate {
        self.state.state
    }

    ///
    #[inline]
    pub const fn as_ptr(&self) -> *const base64_encodestate {
        &self.state.state
    }

    ///
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut base64_encodestate {
        &mut self.state.state
    }

    #[doc(hidden)]
    fn parse(&mut self) {
        let state = self.raw_parse();
        let raw = state.result;
        unsafe {
            let slice = ::core::ffi::CStr::from_ptr(raw);
            let byte = slice.to_bytes();
            let mut res: Vec<u8> = Vec::with_capacity(byte.len());
            res.extend_from_slice(byte);
            self.parsed = Some(res);
            self.new = false;
        }
    }

    ///
    #[inline]
    pub fn as_boxed_slice(&mut self) -> Box<[u8]> {
        if !self.new {
            if self.parsed.is_none() {
                self.parse();
            }
        }

        if self.new {
            self.parse();
        }

        if let Some(msg) = self.parsed.clone() {
            msg.into_boxed_slice()
        } else {
            panic!("No valid matches found, state was broken");
        }
    }

    ///
    #[inline]
    pub fn as_bytes(&mut self) -> &[u8] {
        Box::leak(self.as_boxed_slice())
    }

    ///
    #[inline]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        Box::leak(self.as_boxed_slice())
    }

    ///
    #[inline]
    pub fn as_str(&mut self) -> &str {
        match ::core::str::from_utf8(self.as_bytes()) {
            Ok(s) => s,
            Err(e) => panic!("Invalid UTF-8 sequence: {}. State can't be parsed", e),
        }
    }

    ///
    pub unsafe fn as_str_unchecked(&mut self) -> &str {
        ::core::str::from_utf8_unchecked(self.as_bytes())
    }

    ///
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        match ::core::str::from_utf8_mut(self.as_mut_bytes()) {
            Ok(s) => s,
            Err(e) => panic!("Invalid UTF-8 sequence: {}. State can't be parsed", e),
        }
    }

    ///
    pub unsafe fn as_mut_str_unchecked(&mut self) -> &mut str {
        ::core::str::from_utf8_unchecked_mut(self.as_mut_bytes())
    }

    ///
    #[inline]
    pub fn as_boxed_str(&mut self) -> Box<str> {
        unsafe {
            ::std::str::from_boxed_utf8_unchecked(self.as_boxed_slice())
        }
    }

}