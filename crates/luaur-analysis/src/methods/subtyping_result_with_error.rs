use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_error::TypeError;

impl SubtypingResult {
    pub fn with_error(&mut self, err: TypeError) -> &mut Self {
        self.errors.push(err);
        self
    }
}
