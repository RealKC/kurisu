use std::fmt;
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Unknown = 0,
    Return = 1,
    Constant = 2,
    ConstantLong = 3,
    Negate,
}

impl From<u8> for OpCode {
    fn from(orig: u8) -> Self {
        match orig {
            0x1 => Self::Return,
            0x2 => Self::Constant,
            0x3 => Self::ConstantLong,
            0x4 => Self::Negate,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
