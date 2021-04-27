use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub inner_error: Box<Option<Error>>,
}

impl Error {
    pub fn new(message: &str) -> Error {
        return Error {
            message: message.to_string(),
            inner_error: Box::new(None),
        };
    }

    pub fn with_inner_error(mut self, error: &dyn Display) -> Error {
        let message = format!("{}", error);
        self.inner_error = Box::new(Some(Error::new(&message)));
        return self;
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.inner_error.is_some() {
            return write!(
                f,
                "{} -> {}",
                self.message,
                self.inner_error.clone().unwrap()
            );
        } else {
            return write!(f, "{}", self.message);
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
