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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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
            return Self::OK;
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