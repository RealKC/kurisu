use std::fmt;
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Unknown = 0,
    Return = 1,
    Constant,
}

impl From<u8> for OpCode {
    fn from(orig: u8) -> Self {
        match orig {
            0x1 => Self::Return,
            0x2 => Self::Constant,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
