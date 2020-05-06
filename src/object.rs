use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    String(String),
}

impl Object {
    pub fn is_string(&self) -> bool {
        match self {
            Object::String(_) => true,
            _ => false,
        }
    }

    pub fn as_string(&self) -> &str {
        match self {
            Object::String(stri) => stri,
            _ => panic!("self was not a String"),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(stri) => write!(f, "{}", stri),
        }
    }
}
