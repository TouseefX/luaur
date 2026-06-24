use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::lazy_type::LazyType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::no_refine_type::NoRefineType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TypeVariant {
    Bound(TypeId),
    Error(crate::records::unifiable::Error<TypeId>),
    Free(FreeType),
    Generic(GenericType),
    Primitive(PrimitiveType),
    Singleton(SingletonType),
    Blocked(BlockedType),
    PendingExpansion(PendingExpansionType),
    Function(FunctionType),
    Table(TableType),
    Metatable(MetatableType),
    Extern(ExternType),
    Any(AnyType),
    Union(UnionType),
    Intersection(IntersectionType),
    Lazy(LazyType),
    Unknown(UnknownType),
    Never(NeverType),
    Negation(NegationType),
    NoRefine(NoRefineType),
    TypeFunctionInstance(TypeFunctionInstanceType),
}

impl TypeVariant {
    pub fn index(&self) -> i32 {
        match self {
            TypeVariant::Bound(_) => 0,
            TypeVariant::Error(_) => 1,
            TypeVariant::Free(_) => 2,
            TypeVariant::Generic(_) => 3,
            TypeVariant::Primitive(_) => 4,
            TypeVariant::Singleton(_) => 5,
            TypeVariant::Blocked(_) => 6,
            TypeVariant::PendingExpansion(_) => 7,
            TypeVariant::Function(_) => 8,
            TypeVariant::Table(_) => 9,
            TypeVariant::Metatable(_) => 10,
            TypeVariant::Extern(_) => 11,
            TypeVariant::Any(_) => 12,
            TypeVariant::Union(_) => 13,
            TypeVariant::Intersection(_) => 14,
            TypeVariant::Lazy(_) => 15,
            TypeVariant::Unknown(_) => 16,
            TypeVariant::Never(_) => 17,
            TypeVariant::Negation(_) => 18,
            TypeVariant::NoRefine(_) => 19,
            TypeVariant::TypeFunctionInstance(_) => 20,
        }
    }
}

/// The Rust shape of C++ `Luau::get_if<T>(&TypeVariant)`: one trait impl per
/// alternative lets `get::<T>(ty)` keep the C++ call shape at every call site.
pub trait TypeVariantMember: Sized {
    fn get_if(v: &TypeVariant) -> Option<&Self>;
    fn get_if_mut(v: &mut TypeVariant) -> Option<&mut Self>;
}

macro_rules! type_variant_member {
    ($variant:ident, $ty:ty) => {
        impl TypeVariantMember for $ty {
            fn get_if(v: &TypeVariant) -> Option<&Self> {
                match v {
                    TypeVariant::$variant(inner) => Some(inner),
                    _ => None,
                }
            }
            fn get_if_mut(v: &mut TypeVariant) -> Option<&mut Self> {
                match v {
                    TypeVariant::$variant(inner) => Some(inner),
                    _ => None,
                }
            }
        }
    };
}

type_variant_member!(Error, crate::records::unifiable::Error<TypeId>);
type_variant_member!(Free, FreeType);
type_variant_member!(Generic, GenericType);
type_variant_member!(Primitive, PrimitiveType);
type_variant_member!(Singleton, SingletonType);
type_variant_member!(Blocked, BlockedType);
type_variant_member!(PendingExpansion, PendingExpansionType);
type_variant_member!(Function, FunctionType);
type_variant_member!(Table, TableType);
type_variant_member!(Metatable, MetatableType);
type_variant_member!(Extern, ExternType);
type_variant_member!(Any, AnyType);
type_variant_member!(Union, UnionType);
type_variant_member!(Intersection, IntersectionType);
type_variant_member!(Lazy, LazyType);
type_variant_member!(Unknown, UnknownType);
type_variant_member!(Never, NeverType);
type_variant_member!(Negation, NegationType);
type_variant_member!(NoRefine, NoRefineType);
type_variant_member!(TypeFunctionInstance, TypeFunctionInstanceType);

/// BoundType = Bound<TypeId>; the Bound alternative stores the bare TypeId.
impl TypeVariantMember for crate::type_aliases::bound_type::BoundType {
    fn get_if(v: &TypeVariant) -> Option<&Self> {
        match v {
            TypeVariant::Bound(inner) => {
                // Bound<TypeId> is repr-compatible with its single TypeId field.
                Some(unsafe { &*(inner as *const TypeId as *const Self) })
            }
            _ => None,
        }
    }
    fn get_if_mut(v: &mut TypeVariant) -> Option<&mut Self> {
        match v {
            TypeVariant::Bound(inner) => Some(unsafe { &mut *(inner as *mut TypeId as *mut Self) }),
            _ => None,
        }
    }
}
