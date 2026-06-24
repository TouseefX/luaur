use crate::records::to_string_span::ToStringSpan;
use crate::type_aliases::type_id::TypeId;

impl ToStringSpan {
    pub fn r#type(&self) -> TypeId {
        self.r#type
    }
}
