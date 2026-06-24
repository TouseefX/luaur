use crate::functions::fresh_index::fresh_index;
use crate::records::error::Error;

impl Error {
    pub fn error_id_error() -> Self {
        Error {
            index: fresh_index(),
            synthetic: None,
        }
    }
}
