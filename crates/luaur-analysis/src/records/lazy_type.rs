use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct LazyType {
    pub(crate) unwrap: Option<fn(&mut LazyType)>,
    pub(crate) unwrapped: *const crate::records::r#type::Type,
}
