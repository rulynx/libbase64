use std::mem::ManuallyDrop;

use super::code::{DecodeStatus, EncodeStatus};
use crate::core::error::Error;
use crate::core::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct EncodeKind {
    status: EncodeStatus,
    msg: Option<Vec<u8>>,
}

impl From<EncodeStatus> for EncodeKind {
    fn from(value: EncodeStatus) -> Self {
        EncodeKind { status: value.clone(), msg: Some(value.to_vec()) }
    }
}

impl From<isize> for EncodeKind {
    fn from(value: isize) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

impl From<usize> for EncodeKind {
    fn from(value: usize) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

impl From<i16> for EncodeKind {
    fn from(value: i16) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

impl From<u16> for EncodeKind {
    fn from(value: u16) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<i32> for EncodeKind {
    fn from(value: i32) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<u32> for EncodeKind {
    fn from(value: u32) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<i64> for EncodeKind {
    fn from(value: i64) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u64> for EncodeKind {
    fn from(value: u64) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<i128> for EncodeKind {
    fn from(value: i128) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u128> for EncodeKind {
    fn from(value: u128) -> Self {
        let state = EncodeStatus::from(value);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DecodeKind {
    status: DecodeStatus,
    msg: Option<Vec<u8>>,
}

impl From<DecodeStatus> for DecodeKind {
    fn from(value: DecodeStatus) -> Self {
        DecodeKind { status: value.clone(), msg: Some(value.to_vec()) }
    }
}

impl From<isize> for DecodeKind {
    fn from(value: isize) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

impl From<usize> for DecodeKind {
    fn from(value: usize) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

impl From<i16> for DecodeKind {
    fn from(value: i16) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

impl From<u16> for DecodeKind {
    fn from(value: u16) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<i32> for DecodeKind {
    fn from(value: i32) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<u32> for DecodeKind {
    fn from(value: u32) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<i64> for DecodeKind {
    fn from(value: i64) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u64> for DecodeKind {
    fn from(value: u64) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<i128> for DecodeKind {
    fn from(value: i128) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u128> for DecodeKind {
    fn from(value: u128) -> Self {
        let state = DecodeStatus::from(value);
        DecodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }
}

impl ::core::ops::Deref for EncodeKind {
    type Target = EncodeStatus;

    fn deref(&self) -> &Self::Target {
        &self.status
    }
}

impl ::core::ops::DerefMut for EncodeKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.status
    }
}

unsafe impl ::core::ops::DerefPure for EncodeKind {}

impl ::core::ops::Deref for DecodeKind {
    type Target = DecodeStatus;

    fn deref(&self) -> &Self::Target {
        &self.status
    }
}

impl ::core::ops::DerefMut for DecodeKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.status
    }
}

unsafe impl ::core::ops::DerefPure for DecodeKind {}

impl EncodeKind {

    #[inline]
    pub fn new(code: isize, msg: &'static str) -> EncodeKind {
        EncodeKind { status: EncodeStatus::from(code), msg: Some(msg.as_bytes().to_vec()) }
    }

    #[inline]
    pub const fn is_ok(&self) -> bool {
        self.status.is_ok()
    }

    #[inline]
    pub const fn is_error(&self) -> bool {
        self.status.is_error()
    }

    #[inline]
    pub const fn is_unknown(&self) -> bool {
        self.status.is_unknown()
    }

    #[inline]
    pub const fn code(&self) -> isize {
        self.status.code()
    }

    #[inline]
    pub fn into_boxed_status(self) -> Box<EncodeStatus> {
        self.status.into_boxed_status()
    }

    #[inline]
    pub fn from_boxed_status(status: Box<EncodeStatus>) -> EncodeKind {
        let state = Box::leak(status);
        EncodeKind { status: state.clone(), msg: Some(state.to_vec()) }
    }

    #[inline]
    pub fn handle_abort(&self) -> ! {
        ::std::process::abort();
    }

    #[inline]
    pub fn handle_panic(&self) -> ! {
        if let Some(msg) = self.msg.clone() {
            panic!("{}", String::from_utf8_lossy(&msg));
        }
        panic!("{}", self.status);
    }

    #[inline]
    pub fn handle_exit(&self) -> ! {
        #[cfg(not(target_pointer_width = "16"))]
        {
            ::std::process::exit(self.status.code() as i32);
        }
        #[cfg(target_pointer_width = "16")]
        {
            ::std::process::exit(self.status.code() as i16);
        }
    }

}