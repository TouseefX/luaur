use crate::records::constraint::Constraint;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::variant::Variant3;

pub type ConstraintBlockTarget = Variant3<TypeId, TypePackId, *const Constraint>;
