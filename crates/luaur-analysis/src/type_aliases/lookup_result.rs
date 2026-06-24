use crate::records::not_bindable::NotBindable;
use crate::records::unmapped::Unmapped;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::variant::Variant3;

pub type LookupResult = Variant3<TypePackId, Unmapped, NotBindable>;
