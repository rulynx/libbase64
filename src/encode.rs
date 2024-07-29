use crate::libbase64_sys::{
    base64_encodestep,
    base64_encodestate,
    base64_init_encodestate,
    base64_encode_value,
    base64_encode_block,
    base64_encode_blockend
};

use crate::libc;
use crate::libbase64_derive::{Send, Sync};

#[derive(Debug, Clone, Send, Sync)]
pub struct EncodeErrorKind {
    state: base64_encodestate,
    step: base64_encodestep,
    code: isize,
}

#[derive(Send, Sync)]
pub struct EncodeError {
    msg: &'static str,
    kind: EncodeErrorKind,
}

impl EncodeErrorKind {

    #[inline]
    pub const fn new(state: base64_encodestate, step: base64_encodestep, code: isize) -> EncodeErrorKind {
        EncodeErrorKind { state, step, code }
    }

    #[inline]
    pub const fn from_state(state: base64_encodestate, code: isize) -> EncodeErrorKind {
        EncodeErrorKind { state, step: state.step, code }
    }

    #[inline]
    pub const fn from_state_def(state: base64_encodestate) -> Option<EncodeErrorKind> {
        let step = state.step;

        if ::core::mem::size_of_val::<base64_encodestep>(&step) == 0 {
            return None;
        }

        match step.clone() {
            base64_encodestep::step_A => Some(EncodeErrorKind { state, step, code: 3 }),
            base64_encodestep::step_B => Some(EncodeErrorKind { state, step, code: 2 }),
            base64_encodestep::step_C => Some(EncodeErrorKind { state, step, code: 1}),
        }
    }

    #[inline]
    #[must_use = "'self' will be dropped, if the result is not used"]
    pub const fn into_step(self) -> base64_encodestep {
        self.step
    }

    #[inline]
    pub const fn as_step(&self) -> &base64_encodestep {
        &self.step
    }

    #[inline]
    #[must_use = "'self' will be dropped, if the result is not used"]
    pub const fn into_state(self) -> base64_encodestate {
        self.state
    }

    #[inline]
    pub const fn as_state(&self) -> &base64_encodestate {
        &self.state
    }

    #[inline]
    #[must_use = "'self' will be dropped, if the result is not used"]
    pub const fn into_code(self) -> isize {
        self.code
    }

    #[inline]
    pub const fn as_code(&self) -> &isize {
        &self.code
    }

}