use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TypePackVariant {
    Bound(TypePackId),
    Error(crate::records::unifiable::Error<TypePackId>),
    Free(FreeTypePack),
    Generic(GenericTypePack),
    TypePack(TypePack),
    Variadic(VariadicTypePack),
    Blocked(BlockedTypePack),
    TypeFunctionInstance(TypeFunctionInstanceTypePack),
}

/// Mirror of `TypeVariantMember` for packs: the Rust shape of C++
/// `get_if<T>(&TypePackVariant)`.
pub trait TypePackVariantMember: Sized {
    fn get_if(v: &TypePackVariant) -> Option<&Self>;
    fn get_if_mut(v: &mut TypePackVariant) -> Option<&mut Self>;
}

macro_rules! type_pack_variant_member {
    ($variant:ident, $ty:ty) => {
        impl TypePackVariantMember for $ty {
            fn get_if(v: &TypePackVariant) -> Option<&Self> {
                match v {
                    TypePackVariant::$variant(inner) => Some(inner),
                    _ => None,
                }
            }
            fn get_if_mut(v: &mut TypePackVariant) -> Option<&mut Self> {
                match v {
                    TypePackVariant::$variant(inner) => Some(inner),
                    _ => None,
                }
            }
        }
    };
}

type_pack_variant_member!(Error, crate::records::unifiable::Error<TypePackId>);
type_pack_variant_member!(Free, FreeTypePack);
type_pack_variant_member!(Generic, GenericTypePack);
type_pack_variant_member!(TypePack, TypePack);
type_pack_variant_member!(Variadic, VariadicTypePack);
type_pack_variant_member!(Blocked, BlockedTypePack);
type_pack_variant_member!(TypeFunctionInstance, TypeFunctionInstanceTypePack);

/// BoundTypePack = Bound<TypePackId>; the Bound alternative stores the bare id.
impl TypePackVariantMember for crate::type_aliases::bound_type_pack::BoundTypePack {
    fn get_if(v: &TypePackVariant) -> Option<&Self> {
        match v {
            TypePackVariant::Bound(inner) => {
                Some(unsafe { &*(inner as *const TypePackId as *const Self) })
            }
            _ => None,
        }
    }
    fn get_if_mut(v: &mut TypePackVariant) -> Option<&mut Self> {
        match v {
            TypePackVariant::Bound(inner) => {
                Some(unsafe { &mut *(inner as *mut TypePackId as *mut Self) })
            }
            _ => None,
        }
    }
}
