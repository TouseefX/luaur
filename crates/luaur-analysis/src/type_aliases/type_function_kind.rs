use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use luaur_common::records::variant::Variant2;

pub type TypeFunctionKind = Variant2<TypeFunctionTypeId, TypeFunctionTypePackId>;

/// `get_if<T>(&tfkind)` over the kind variant (TypeFunctionRuntimeBuilder.h:12).
pub trait TypeFunctionKindMember: Sized {
    fn get_if(v: &TypeFunctionKind) -> Option<&Self>;
    fn get_if_mut(v: &mut TypeFunctionKind) -> Option<&mut Self>;
}

impl TypeFunctionKindMember for TypeFunctionTypeId {
    fn get_if(v: &TypeFunctionKind) -> Option<&Self> {
        v.get_if_0()
    }
    fn get_if_mut(v: &mut TypeFunctionKind) -> Option<&mut Self> {
        v.get_if_0_mut()
    }
}

impl TypeFunctionKindMember for TypeFunctionTypePackId {
    fn get_if(v: &TypeFunctionKind) -> Option<&Self> {
        v.get_if_1()
    }
    fn get_if_mut(v: &mut TypeFunctionKind) -> Option<&mut Self> {
        v.get_if_1_mut()
    }
}
