use std::hash::Hash;

pub struct Type<'a> {
    pub mime: &'a str,
    pub extension: &'a str,
}

pub const fn new_type<'a>(mime: &'a str, extension: &'a str) -> Type<'a> {
    Type { mime, extension }
}

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
