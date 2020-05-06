use std::fmt;

use crate::object::Object;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    Obj(Box<Object>),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Self::Boolean(b) => !b,
            Self::Nil => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Self::Number(_) => true,
            _ => false,
        }
    }

    pub fn to_number(&self) -> f64 {
        match self {
            Self::Number(n) => *n,
            _ => panic!("self was not a number"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
            Value::Obj(_) => todo!(),
        }
    }
}
