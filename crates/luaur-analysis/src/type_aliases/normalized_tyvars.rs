use crate::records::normalized_type::NormalizedType;
use crate::type_aliases::type_id::TypeId;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;

pub type NormalizedTyvars = BTreeMap<TypeId, Box<NormalizedType>>;
