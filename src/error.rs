use std::fmt;

pub struct Error {
    error_msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ file: {}, line: {}, msg: {} }}",
            file!(),
            line!(),
            self.error_msg
        )
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Error {
        Error {
            error_msg: value.to_string(),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Error {
        Error {
            error_msg: value,
        }
    }
}
