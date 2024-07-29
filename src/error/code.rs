#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub enum EncodeStatus {
    OK,
    ERROR,
    UNKNOWN
}

impl EncodeStatus {

    #[inline]
    #[must_use = "'self' will be dropped, if the resulted box not used"]
    pub fn into_boxed_status(self) -> Box<EncodeStatus> {
        Box::new(self)
    }

    #[inline]
    pub const fn is_ok(&self) -> bool {
        match self {
            Self::OK => true,
            Self::ERROR => false,
            Self::UNKNOWN => false,
        }
    }

    #[inline]
    pub const fn is_error(&self) -> bool {
        match self {
            Self::OK => false,
            Self::ERROR => true,
            Self::UNKNOWN => false,
        }
    }

    #[inline]
    pub const fn is_unknown(&self) -> bool {
        match self {
            Self::OK => false,
            Self::ERROR => false,
            Self::UNKNOWN => true,
        }
    }

    #[inline]
    pub const fn code(&self) -> isize {
        match self {
            Self::OK => 0,
            Self::ERROR => 1,
            Self::UNKNOWN => 2,
        }
    }

}

impl From<isize> for EncodeStatus {
    fn from(value: isize) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN
        }
    }
}

impl From<usize> for EncodeStatus {
    fn from(value: usize) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN
        } 
    }
}

impl From<i16> for EncodeStatus {
    fn from(value: i16) -> Self {
        Self::from(isize::from(value))
    }
}

impl From<u16> for EncodeStatus {
    fn from(value: u16) -> Self {
        Self::from(usize::from(value))
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<i32> for EncodeStatus {
    fn from(value: i32) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<u32> for EncodeStatus {
    fn from(value: u32) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<i64> for EncodeStatus {
    fn from(value: i64) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }  
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u64> for EncodeStatus {
    fn from(value: u64) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }  
    }
}

#[cfg(target_pointer_width = "64")]
impl From<i128> for EncodeStatus {
    fn from(value: i128) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }  
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u128> for EncodeStatus {
    fn from(value: u128) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }  
    }
}

#[derive(Debug, Clone)]
pub enum DecodeStatus {
    OK,
    ERROR,
    UNKNOWN
}

impl DecodeStatus {

    #[inline]
    #[must_use = "'self' will be dropped, if the resulted box not used"]
    pub fn into_boxed_status(self) -> Box<DecodeStatus> {
        Box::new(self)
    }

    #[inline]
    pub const fn is_ok(&self) -> bool {
        match self {
            Self::OK => true,
            Self::ERROR => false,
            Self::UNKNOWN => false,
        }
    }

    #[inline]
    pub const fn is_error(&self) -> bool {
        match self {
            Self::OK => false,
            Self::ERROR => true,
            Self::UNKNOWN => false,
        }
    }

    #[inline]
    pub const fn is_unknown(&self) -> bool {
        match self {
            Self::OK => false,
            Self::ERROR => false,
            Self::UNKNOWN => true,
        }
    }

    #[inline]
    pub const fn code(&self) -> isize {
        match self {
            Self::OK => 0,
            Self::ERROR => 1,
            Self::UNKNOWN => 2,
        }
    }

}

impl From<isize> for DecodeStatus {
    fn from(value: isize) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN
        }
    }
}

impl From<usize> for DecodeStatus {
    fn from(value: usize) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN
        } 
    }
}

impl From<i16> for DecodeStatus {
    fn from(value: i16) -> Self {
        Self::from(isize::from(value))
    }
}

impl From<u16> for DecodeStatus {
    fn from(value: u16) -> Self {
        Self::from(usize::from(value))
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<i32> for DecodeStatus {
    fn from(value: i32) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl From<u32> for DecodeStatus {
    fn from(value: u32) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl From<i64> for DecodeStatus {
    fn from(value: i64) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }  
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u64> for DecodeStatus {
    fn from(value: u64) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }  
    }
}

#[cfg(target_pointer_width = "64")]
impl From<i128> for DecodeStatus {
    fn from(value: i128) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }  
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u128> for DecodeStatus {
    fn from(value: u128) -> Self {
        if value == 0 {
            Self::OK
        } else if value == 1 {
            return Self::ERROR;
        } else {
            return Self::UNKNOWN;
        }  
    }
}

impl Default for EncodeStatus {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl Default for DecodeStatus {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl ::core::fmt::Display for EncodeStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::OK => "OK",
            Self::ERROR => "ERROR",
            Self::UNKNOWN => "UNKNOWN",
        })
    }
}

impl ::core::fmt::Display for DecodeStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::OK => "OK",
            Self::ERROR => "ERROR",
            Self::UNKNOWN => "UNKNOWN",
        })
    }
}


impl ::core::error::Error for EncodeStatus {
    fn description(&self) -> &str {
        match self {
            Self::OK => "OK",
            Self::ERROR => "ERROR",
            Self::UNKNOWN => "UNKNOWN",
        }
    }
}

impl ::core::error::Error for DecodeStatus {
    fn description(&self) -> &str {
        match self {
            Self::OK => "OK",
            Self::ERROR => "ERROR",
            Self::UNKNOWN => "UNKNOWN",
        }
    }
}

impl ::core::ops::Deref for EncodeStatus {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::OK => b"OK",
            Self::ERROR => b"ERROR",
            Self::UNKNOWN => b"UNKNOWN",
        }
    }
}

impl ::core::ops::DerefMut for EncodeStatus {

    fn deref_mut(&mut self) -> &mut Self::Target {
        let mut res = self.deref().to_vec();
        let boxed = res.into_boxed_slice();
        Box::leak(boxed)
    }
}

impl ::core::ops::Deref for DecodeStatus {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::OK => b"OK",
            Self::ERROR => b"ERROR",
            Self::UNKNOWN => b"UNKNOWN",
        }
    }
}

impl ::core::ops::DerefMut for DecodeStatus {
    
    fn deref_mut(&mut self) -> &mut Self::Target {
        let mut res = self.deref().to_vec();
        let boxed = res.into_boxed_slice();
        Box::leak(boxed)
    }
}

unsafe impl ::core::ops::DerefPure for EncodeStatus {}
unsafe impl ::core::ops::DerefPure for DecodeStatus {}

impl AsRef<[u8]> for EncodeStatus {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl AsMut<[u8]> for EncodeStatus {
    fn as_mut(&mut self) -> &mut [u8] {
        self.deref_mut()
    }
}

impl AsRef<[u8]> for DecodeStatus {
    fn as_ref(&self) -> &[u8] {
        self.deref()
    }
}

impl AsMut<[u8]> for DecodeStatus {
    fn as_mut(&mut self) -> &mut [u8] {
        self.deref_mut()
    }
}

impl AsRef<str> for EncodeStatus {
    fn as_ref(&self) -> &str {
        match ::core::str::from_utf8(self.as_ref()) {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    }
}

impl AsMut<str> for EncodeStatus {
    fn as_mut(&mut self) -> &mut str {
        match ::core::str::from_utf8_mut(self.as_mut()) {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    }
}

impl AsRef<str> for DecodeStatus {
    fn as_ref(&self) -> &str {
        match ::core::str::from_utf8(self.as_ref()) {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    }
}

impl AsMut<str> for DecodeStatus {
    fn as_mut(&mut self) -> &mut str {
        match ::core::str::from_utf8_mut(self.as_mut()) {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    }
}