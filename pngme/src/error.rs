use std::fmt;

#[derive(Debug)]
pub struct DataError;

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is something wrong with the data")
    }
}

impl std::error::Error for DataError {}