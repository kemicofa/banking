pub struct Error(&'static str);

impl Error {
    pub fn new(message: &str) -> Self {
        Self(message)
    }
}