use crate::records::type_error::TypeError;

impl TypeError {
    pub fn summary(&self) -> crate::records::type_error_summary::TypeErrorSummary {
        crate::records::type_error_summary::TypeErrorSummary::type_error_summary_type_error_summary(
            self.location,
            self.module_name.clone(),
            self.code(),
        )
    }
}
