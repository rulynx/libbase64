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

impl StateParser {
    
    ///
    #[inline]
    pub fn new() -> StateParser {
        let state = EncodeState::new();
        StateParser { state: state, parsed: None, new: true }
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

}