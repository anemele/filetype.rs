pub struct Type {
    pub mime: String,
    pub extension: String,
}

pub trait Match {
    fn r#match(buf: Vec<u8>) -> bool;
}

impl Type {
    pub fn new(mime: &str, extension: &str) -> Self {
        Type {
            mime: String::from(mime),
            extension: String::from(extension),
        }
    }

    pub fn is_mime(&self, mime: &str) -> bool {
        self.mime == mime
    }

    pub fn is_extension(&self, extension: &str) -> bool {
        self.extension == extension
    }
}
