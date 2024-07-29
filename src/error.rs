pub(crate) mod code;
pub(crate) mod kind;

pub use code::{DecodeStatus, EncodeStatus};
pub use kind::{DecodeKind, EncodeKind};

#[derive(Debug, Clone, Default)]
pub struct EncodeError {
    pub(crate) msg: Vec<u8>,
    pub(crate) kind: EncodeKind,
}

impl EncodeError {

    #[inline]
    pub fn new_custom(msg: &'static str, code: isize) -> EncodeError {
        let msg = msg.as_bytes().to_vec();
        let kind = EncodeKind::from(code);
        EncodeError { msg, kind }
    }

    #[inline]
    pub fn new_from_code(code: isize) -> EncodeError {
        let kind = EncodeKind::from(code);
        let msg = kind.msg.clone().unwrap_or("Can't convert to base64".as_bytes().to_vec());
        EncodeError { msg, kind }
    }

    #[inline]
    pub fn new_from_kind(kind: EncodeKind) -> EncodeError {
        let msg = kind.msg.clone().unwrap_or("Can't convert to base64".as_bytes().to_vec());
        EncodeError { msg, kind }
    }

    #[inline]
    pub fn new_from_status(status: EncodeStatus) -> EncodeError {
        let kind = EncodeKind::from(status);
        let msg = kind.msg.clone().unwrap_or("Can't convert to base64".as_bytes().to_vec());
        EncodeError { msg, kind }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DecodeError {
    msg: Vec<u8>,
    kind: DecodeKind,
}