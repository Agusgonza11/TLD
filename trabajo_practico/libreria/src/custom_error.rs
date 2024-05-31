use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    Err,
    AccionInvalida,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::Err => write!(f, ""),
            CustomError::AccionInvalida=> write!(f, "Error: Acción invalida"),
        }
    }
}
