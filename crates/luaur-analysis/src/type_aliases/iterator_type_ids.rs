use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
pub type iterator = core::slice::IterMut<'static, TypeId>;
