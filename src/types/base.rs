use std::{collections::HashMap, fmt, hash::Hash};

pub struct Type<'a> {
    pub mime: &'a str,
    pub extension: &'a str,
}

pub const fn new_type<'a>(mime: &'a str, extension: &'a str) -> Type<'a> {
    Type { mime, extension }
}

const TYPE_UNKNOWN_STR: &str = "UNKNOWN";
pub const TYPE_UNKNOWN: Type = new_type(TYPE_UNKNOWN_STR, "");

impl Hash for Type<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.mime.hash(state);
        self.extension.hash(state);
    }
}

impl Eq for Type<'_> {}

impl PartialEq for Type<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.mime == other.mime && self.extension == other.extension
    }
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == TYPE_UNKNOWN {
            write!(f, "{}", TYPE_UNKNOWN_STR)
        } else {
            write!(f, "{} ({})", self.mime, self.extension)
        }
    }
}

pub type TypeMatcher = fn(&[u8]) -> bool;
pub type TypeTypesMatcher = HashMap<Type<'static>, TypeMatcher>;
