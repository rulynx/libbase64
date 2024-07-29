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

#[derive(Debug, Clone, Default)]
pub struct DecodeKind {
    status: DecodeStatus,
    msg: Option<Vec<u8>>,
}