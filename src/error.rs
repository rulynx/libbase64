pub(crate) mod code;
pub(crate) mod kind;

use core::error::Error;

pub use crate::error::code::{DecodeStatus, EncodeStatus};
pub use crate::error::kind::{DecodeKind, EncodeKind};

#[derive(Debug, Clone)]
pub struct EncodeError {
    pub(crate) msg: Option<Vec<u8>>,
    pub(crate) kind: EncodeKind,
}

impl core::error::Error for EncodeError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => Box::leak(msg.to_string().into_boxed_str()),
                Err(_) => self.kind.description(),
            }
        } else {
            self.kind.description()
        }
    }
}

impl core::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => f.write_str(msg),
                Err(_) => self.kind.fmt(f),
            }
        } else {
            self.kind.fmt(f)
        }
    }
}

impl EncodeError {

    #[inline]
    pub fn new(msg: &'static str, code: isize) -> EncodeError {
        let kind = EncodeKind::from(code);
        EncodeError { msg: Some(msg.as_bytes().to_vec()), kind }
    }

    #[inline]
    #[allow(deprecated)]
    pub fn from_code(code: isize) -> EncodeError {
        let kind = EncodeKind::from(code);
        let msg = kind.description();
        EncodeError { msg: Some(msg.as_bytes().to_vec()), kind }
    }

    #[inline]
    #[allow(deprecated)]
    pub fn from_kind(kind: EncodeKind) -> EncodeError {
        let msg = kind.description();
        EncodeError { msg: Some(msg.as_bytes().to_vec()), kind }
    }

    #[inline]
    pub fn from_code_no_msg(code: isize) -> EncodeError {
        let kind = EncodeKind::from(code);
        EncodeError { msg: None, kind }
    }

    #[inline]
    pub fn from_kind_no_msg(kind: EncodeKind) -> EncodeError {
        EncodeError { msg: None, kind }
    }

    #[inline]
    pub fn handle_print(&self) {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => eprintln!("{}", msg),
                Err(_) => self.kind.handle_print(),
            }
        } else {
            self.kind.handle_print()
        }
    }

    #[inline]
    pub fn handle_panic(&self) -> ! {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => panic!("{}", msg),
                Err(_) => self.kind.handle_panic(),
            }
        } else {
            self.kind.handle_panic()
        }
    }

    #[inline]
    pub fn handle_abort(&self) -> ! {
        self.kind.handle_abort()
    }

    #[inline]
    pub fn handle_exit(&self) -> ! {
        self.kind.handle_exit()
    }

    #[inline]
    pub fn abort_with_msg(&self) -> ! {
        if self.msg.is_some() {
            self.handle_print();
            self.handle_abort();
        } else {
            self.kind.abort_with_msg();
        }
    }

    #[inline]
    pub fn exit_with_msg(&self) -> ! {
        if self.msg.is_some() {
            self.handle_print();
            self.handle_exit();
        } else {
            self.kind.exit_with_msg();
        }
    }

    #[inline]
    pub fn is_status_ok(&self) -> bool {
        self.kind.is_ok()
    }

    #[inline]
    pub fn is_status_error(&self) -> bool {
        self.kind.is_error()
    }

    #[inline]
    pub fn is_status_unknown(&self) -> bool {
        self.kind.is_unknown()
    }

    #[inline]
    pub fn get_status_code(&self) -> isize {
        self.kind.code()
    }

    #[inline]
    pub fn kind(&self) -> EncodeKind {
        self.kind.clone()
    }
}

#[derive(Debug, Clone)]
pub struct DecodeError {
    pub(crate) msg: Option<Vec<u8>>,
    pub(crate) kind: DecodeKind,
}

impl core::error::Error for DecodeError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => Box::leak(msg.to_string().into_boxed_str()),
                Err(_) => self.kind.description(),
            }
        } else {
            self.kind.description()
        }
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => f.write_str(msg),
                Err(_) => self.kind.fmt(f),
            }
        } else {
            self.kind.fmt(f)
        }
    }
}

impl DecodeError {

    #[inline]
    pub fn new(msg: &'static str, code: isize) -> DecodeError {
        let kind = DecodeKind::from(code);
        DecodeError { msg: Some(msg.as_bytes().to_vec()), kind }
    }

    #[inline]
    #[allow(deprecated)]
    pub fn from_code(code: isize) -> DecodeError {
        let kind = DecodeKind::from(code);
        let msg = kind.description();
        DecodeError { msg: Some(msg.as_bytes().to_vec()), kind }
    }

    #[inline]
    #[allow(deprecated)]
    pub fn from_kind(kind: DecodeKind) -> DecodeError {
        let msg = kind.description();
        DecodeError { msg: Some(msg.as_bytes().to_vec()), kind }
    }

    #[inline]
    pub fn from_code_no_msg(code: isize) -> DecodeError {
        let kind = DecodeKind::from(code);
        DecodeError { msg: None, kind }
    }

    #[inline]
    pub fn from_kind_no_msg(kind: DecodeKind) -> DecodeError {
        DecodeError { msg: None, kind }
    }

    #[inline]
    pub fn handle_print(&self) {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => eprintln!("{}", msg),
                Err(_) => self.kind.handle_print(),
            }
        } else {
            self.kind.handle_print()
        }
    }

    #[inline]
    pub fn handle_panic(&self) -> ! {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => panic!("{}", msg),
                Err(_) => self.kind.handle_panic(),
            }
        } else {
            self.kind.handle_panic()
        }
    }

    #[inline]
    pub fn handle_abort(&self) -> ! {
        self.kind.handle_abort()
    }

    #[inline]
    pub fn handle_exit(&self) -> ! {
        self.kind.handle_exit()
    }

    #[inline]
    pub fn abort_with_msg(&self) -> ! {
        if self.msg.is_some() {
            self.handle_print();
            self.handle_abort();
        } else {
            self.kind.abort_with_msg();
        }
    }

    #[inline]
    pub fn exit_with_msg(&self) -> ! {
        if self.msg.is_some() {
            self.handle_print();
            self.handle_exit();
        } else {
            self.kind.exit_with_msg();
        }
    }

    #[inline]
    pub fn is_status_ok(&self) -> bool {
        self.kind.is_ok()
    }

    #[inline]
    pub fn is_status_error(&self) -> bool {
        self.kind.is_error()
    }

    #[inline]
    pub fn is_status_unknown(&self) -> bool {
        self.kind.is_unknown()
    }

    
    #[inline]
    pub fn get_status_code(&self) -> isize {
        self.kind.code()
    }

    #[inline]
    pub fn kind(&self) -> DecodeKind {
        self.kind.clone()
    }
}

impl ::core::ops::Deref for EncodeError {
    type Target = dyn ::core::error::Error;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl ::core::ops::DerefMut for EncodeError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

impl ::core::ops::Deref for DecodeError {
    type Target = dyn ::core::error::Error;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl ::core::ops::DerefMut for DecodeError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

unsafe impl ::core::ops::DerefPure for EncodeError {}
unsafe impl ::core::ops::DerefPure for DecodeError {}

impl AsRef<str> for EncodeError {

    #[allow(deprecated)]
    fn as_ref(&self) -> &str {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => Box::leak(msg.to_string().into_boxed_str()),
                Err(_) => self.kind.description(),
            }
        } else {
            self.kind.description()
        }
    }
}

impl AsMut<str> for EncodeError {
    fn as_mut(&mut self) -> &mut str {
        let or: &str = self.as_ref();
        Box::leak(or.to_string().into_boxed_str())
    }
}


impl AsRef<str> for DecodeError {

    #[allow(deprecated)]
    fn as_ref(&self) -> &str {
        if let Some(msg) = self.msg.clone() {
            match ::core::str::from_utf8(&msg) {
                Ok(msg) => Box::leak(msg.to_string().into_boxed_str()),
                Err(_) => self.kind.description(),
            }
        } else {
            self.kind.description()
        }
    }
}

impl AsMut<str> for DecodeError {
    fn as_mut(&mut self) -> &mut str {
        let or: &str = self.as_ref();
        Box::leak(or.to_string().into_boxed_str())
    }
}
