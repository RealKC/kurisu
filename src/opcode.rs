use std::fmt;
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Unknown = 0,
    Return,
    Constant,
    ConstantLong,
    Nil,
    True,
    False,
    Negate,
    Not,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
}

impl From<u8> for OpCode {
    fn from(orig: u8) -> Self {
        match orig {
            1 => Self::Return,
            2 => Self::Constant,
            3 => Self::ConstantLong,
            4 => Self::Nil,
            5 => Self::True,
            6 => Self::False,
            7 => Self::Negate,
            8 => Self::Not,
            9 => Self::Add,
            10 => Self::Subtract,
            11 => Self::Multiply,
            12 => Self::Divide,
            13 => Self::Equal,
            14 => Self::Greater,
            15 => Self::Less,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
