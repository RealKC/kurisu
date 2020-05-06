use std::fmt;

use crate::object::Object;

#[derive(Debug, Clone, PartialOrd)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    Obj(Box<Object>),
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Value) -> bool {
        match (self, rhs) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Obj(a), Value::Obj(b)) => **a == **b,
            _ => false,
        }
    }
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

    pub fn is_string(&self) -> bool {
        match self {
            Value::Obj(obj) => obj.is_string(),
            _ => false,
        }
    }

    pub fn as_string(&self) -> &str {
        match self {
            Value::Obj(obj) => obj.as_string(),
            _ => panic!("self was not an Object"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::Nil => write!(f, "nil"),
            Value::Obj(obj) => write!(f, "{}", obj),
        }
    }
}
