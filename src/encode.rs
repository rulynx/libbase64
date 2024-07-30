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

/// # [`EncodeState`]
/// 
/// This Struct Bound the [`crate::b64sys::base64_encodestate`] struct and provide a way to create easy and fast an new Instance of it.
/// 
/// # Example
/// ```rust
/// use b64sys::encode::EncodeState;
/// 
/// let state = EncodeState::new();
/// ```
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

    /// # Create a new [`EncodeState`]
    /// 
    /// This function create a new [`EncodeState`] and return it.
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::EncodeState;
    /// 
    /// let state = EncodeState::new();
    /// ```
    #[inline]
    pub fn new() -> EncodeState {
        let mut state_uninit = base64_encodestate { step: base64_encodestep::step_A, stepcount: 0, result: core::ptr::null_mut() };
        unsafe { base64_init_encodestate(&mut state_uninit) };
        EncodeState { state: state_uninit }
    }
}

/// # [`StateParser`]
/// 
/// This Struct Helps to Parse the [`EncodeState`] and the [`crate::b64sys::base64_encodestate`] struct from him.
/// The [`crate::b64sys::base64_encodestate`] struct is a C Struct and can't be parsed directly in Rust.
/// So the [`StateParser`] Struct is here to help to parse the [`crate::b64sys::base64_encodestate`] struct from the [`EncodeState`] struct.
/// 
/// This will give an safe way to works with the 'base64' API from the C language.
/// 
/// The [`StateParser`] can be created from an [`EncodeState`], a [`crate::b64sys::base64_encodestate`] struct or from default with a new  [`EncodeState`].
/// 
/// # Example
/// ```rust
/// use b64sys::encode::{EncodeState, StateParser};
/// 
/// let state = EncodeState::new();
/// let parser = StateParser::from(state);
/// ```
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
    
    /// # Create a new [`StateParser`]
    /// 
    /// This function create a new [`StateParser`] and return it.
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::StateParser;
    /// 
    /// let parser = StateParser::new();
    /// ```
    #[inline]
    pub fn new() -> StateParser {
        let state = EncodeState::new();
        StateParser { state, parsed: None, new: true }
    }

    /// # Parse the [`EncodeState`] to a [`crate::b64sys::base64_encodestate`] struct
    /// 
    /// This function parse the [`EncodeState`] to a [`crate::b64sys::base64_encodestate`] struct and return it.
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let parser = StateParser::from(state);
    /// ```
    #[inline]
    pub const fn raw_parse(&self) -> base64_encodestate {
        self.state.state
    }

    /// # Get the Mutable Raw Pointer to the [`crate::b64sys::base64_encodestate`] struct
    /// 
    /// This function return the Mutable Raw Pointer to the [`crate::b64sys::base64_encodestate`] struct.
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let parser = StateParser::from(state);
    /// let ptr = parser.as_ptr();
    /// ```
    #[inline]
    pub const fn as_ptr(&self) -> *const base64_encodestate {
        &self.state.state
    }

    /// # Get the Mutable Raw Pointer to the [`crate::b64sys::base64_encodestate`] struct
    /// 
    /// This function return the Mutable Raw Pointer to the [`crate::b64sys::base64_encodestate`] struct.
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let parser = StateParser::from(state);
    /// let ptr = parser.as_mut_ptr();
    /// ```
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

    /// # Get the Result, which Contains the [`crate::b64sys::base64_encodestate`] struct as boxed slice.
    /// 
    /// This function return the Result, which Contains the [`crate::b64sys::base64_encodestate`] struct as boxed slice.
    /// 
    /// <div class="warning">
    /// 
    /// This Function Converts indirectly the [`crate::b64sys::base64_encodestate`] struct to a boxed slice.
    /// The Resulted Box is created from an reference of the [`StateParser`] struct.
    /// The Parent [`StateParser`] must be mutable for using this Function
    /// 
    /// </div>
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let mut parser = StateParser::from(state);
    /// let result = parser.as_boxed_slice();
    /// ```
    #[inline]
    pub fn as_boxed_slice(&mut self) -> Box<[u8]> {
        if !self.new && self.parsed.is_none() {
            self.parse();
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

    /// # Get the Result, which Contains the [`crate::b64sys::base64_encodestate`] struct as byte.
    /// 
    /// This function return the Result, which Contains the [`crate::b64sys::base64_encodestate`] struct as byte.
    /// 
    /// <div class="warning">
    /// 
    /// The Same info how by boxed slice.
    /// For More Info [see](#method.as_boxed_slice)
    /// 
    /// </div>
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let mut parser = StateParser::from(state);
    /// let result = parser.as_bytes();
    /// ```
    #[inline]
    pub fn as_bytes(&mut self) -> &[u8] {
        Box::leak(self.as_boxed_slice())
    }

    /// # The Mutable version of [`self::StateParser::as_bytes`]
    /// 
    /// This function return the Mutable version of [`self::StateParser::as_bytes`].
    /// 
    /// <div class="warning">
    /// 
    /// The Same info how by boxed slice.
    /// For More Info [see](#method.as_boxed_slice)
    /// 
    /// </div>
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let mut parser = StateParser::from(state);
    /// let result = parser.as_mut_bytes();
    /// ```
    #[inline]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        Box::leak(self.as_boxed_slice())
    }

    /// # Get the Result, which Contains the [`crate::b64sys::base64_encodestate`] struct as string.
    /// 
    /// This function return the Result, which Contains the [`crate::b64sys::base64_encodestate`] struct as string.
    /// 
    /// <div class="warning">
    /// 
    /// This Function Converts indirectly the [`crate::b64sys::base64_encodestate`] struct to a string.
    /// The Resulted String is created from an reference of the [`StateParser`] struct.
    /// The Parent [`StateParser`] must be mutable for using this Function
    /// 
    /// </div>
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let mut parser = StateParser::from(state);
    /// let result = parser.as_str();
    /// ```
    #[inline]
    pub fn as_str(&mut self) -> &str {
        match ::core::str::from_utf8(self.as_bytes()) {
            Ok(s) => s,
            Err(e) => panic!("Invalid UTF-8 sequence: {}. State can't be parsed", e),
        }
    }

    /// # Get an unchecked version of [`self::StateParser::as_str`]
    /// 
    /// This function return an unchecked version of [`self::StateParser::as_str`].
    /// 
    /// # Safety
    /// 
    /// <div class="warning">
    /// 
    /// This Function is unsafe because it doesn't check if the string is valid UTF-8.
    /// 
    /// </div>
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let mut parser = StateParser::from(state);
    /// let result = parser.as_str_unchecked();
    /// ```
    pub unsafe fn as_str_unchecked(&mut self) -> &str {
        ::core::str::from_utf8_unchecked(self.as_bytes())
    }

    /// # Get a Mutable Version of [`self::StateParser::as_str`]
    /// 
    /// This function return a Mutable Version of [`self::StateParser::as_str`].
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let mut parser = StateParser::from(state);
    /// let result = parser.as_mut_str();
    /// ```
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        match ::core::str::from_utf8_mut(self.as_mut_bytes()) {
            Ok(s) => s,
            Err(e) => panic!("Invalid UTF-8 sequence: {}. State can't be parsed", e),
        }
    }

    /// # Get an unchecked version of [`self::StateParser::as_mut_str`]
    /// 
    /// This function return an unchecked version of [`self::StateParser::as_mut_str`].
    /// 
    /// # Safety
    /// 
    /// <div class="warning">
    /// 
    /// This Function is unsafe because it doesn't check if the string is valid UTF-8.
    /// 
    /// </div>
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let mut parser = StateParser::from(state);
    /// let result = parser.as_mut_str_unchecked();
    /// ```
    pub unsafe fn as_mut_str_unchecked(&mut self) -> &mut str {
        ::core::str::from_utf8_unchecked_mut(self.as_mut_bytes())
    }

    /// # Get the Result, which Contains the [`crate::b64sys::base64_encodestate`] struct as boxed string.
    /// 
    /// This function return the Result, which Contains the [`crate::b64sys::base64_encodestate`] struct as boxed string.
    /// 
    /// # Example
    /// ```rust
    /// use b64sys::encode::{EncodeState, StateParser};
    /// 
    /// let state = EncodeState::new();
    /// let mut parser = StateParser::from(state);
    /// let result = parser.as_boxed_str();
    /// ```
    #[inline]
    pub fn as_boxed_str(&mut self) -> Box<str> {
        unsafe {
            ::std::str::from_boxed_utf8_unchecked(self.as_boxed_slice())
        }
    }

}