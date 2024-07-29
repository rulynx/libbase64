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

#[derive(Debug, Clone, Default)]
pub struct DecodeKind {
    status: DecodeStatus,
    msg: Option<Vec<u8>>,
}