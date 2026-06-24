use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_pack_variant::TypePackVariant;

#[derive(Debug, Clone)]
pub struct TypePackVar {
    pub(crate) ty: TypePackVariant,
    pub(crate) persistent: bool,
    pub(crate) owningArena: *mut TypeArena,
}

/// `TypeArena::add_type_pack_t<T: Into<TypePackVar>>` is designed to accept a
/// bare type-pack variant (mirroring C++ `arena->addTypePack(VariadicTypePack{...})`).
/// These `From` impls wrap each variant struct into a non-persistent
/// `TypePackVar`, exactly as the C++ `TypePackVar(variant)` constructor does.
macro_rules! impl_from_variant_for_type_pack_var {
    ($variant:ident, $ty:path) => {
        impl From<$ty> for TypePackVar {
            fn from(v: $ty) -> Self {
                TypePackVar {
                    ty: TypePackVariant::$variant(v),
                    persistent: false,
                    owningArena: core::ptr::null_mut(),
                }
            }
        }
    };
}

impl_from_variant_for_type_pack_var!(Free, crate::records::free_type_pack::FreeTypePack);
impl_from_variant_for_type_pack_var!(Error, crate::type_aliases::error_type_pack::ErrorTypePack);
impl_from_variant_for_type_pack_var!(Generic, crate::records::generic_type_pack::GenericTypePack);
impl_from_variant_for_type_pack_var!(TypePack, crate::records::type_pack::TypePack);
impl_from_variant_for_type_pack_var!(
    Variadic,
    crate::records::variadic_type_pack::VariadicTypePack
);
impl_from_variant_for_type_pack_var!(Blocked, crate::records::blocked_type_pack::BlockedTypePack);
impl_from_variant_for_type_pack_var!(
    TypeFunctionInstance,
    crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack
);

impl TypePackVar {
    pub fn new(ty: TypePackVariant) -> Self {
        Self {
            ty,
            persistent: false,
            owningArena: core::ptr::null_mut(),
        }
    }

    pub fn new_with_persistence(ty: TypePackVariant, persistent: bool) -> Self {
        Self {
            ty,
            persistent,
            owningArena: core::ptr::null_mut(),
        }
    }

    pub fn is_persistent(&self) -> bool {
        self.persistent
    }

    pub fn owning_arena(&self) -> *mut TypeArena {
        self.owningArena
    }
}

impl From<crate::type_aliases::bound_type_pack::BoundTypePack> for TypePackVar {
    fn from(v: crate::type_aliases::bound_type_pack::BoundTypePack) -> Self {
        TypePackVar {
            ty: TypePackVariant::Bound(v.boundTo),
            persistent: false,
            owningArena: core::ptr::null_mut(),
        }
    }
}
