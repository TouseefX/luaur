use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_variant::TypeVariant;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct Type {
    pub ty: TypeVariant,
    /// Kludge: A persistent Type is one that belongs to the global scope.
    /// Global type bindings are immutable but are reused many times.
    /// Persistent Types do not get cloned.
    pub persistent: bool,
    pub documentation_symbol: Option<String>,
    /// Pointer to the type arena that allocated this type.
    pub owning_arena: *mut TypeArena,
}

impl Type {
    pub(crate) fn new(ty: TypeVariant) -> Self {
        Self {
            ty,
            persistent: false,
            documentation_symbol: None,
            owning_arena: core::ptr::null_mut(),
        }
    }

    pub(crate) fn new_with_persistence(ty: TypeVariant, persistent: bool) -> Self {
        Self {
            ty,
            persistent,
            documentation_symbol: None,
            owning_arena: core::ptr::null_mut(),
        }
    }
}

/// `Type::operator==` delegates to `areEqual` (structural equality) in C++.
/// Stub — awaiting `areEqual` port. Two `Type` pointers (TypeId) are equal iff they are the
/// same address; structural equality is separate and handled via `areEqual`.
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        // Pointer identity: this matches the most common usage (TypeId == TypeId).
        core::ptr::eq(self, other)
    }
}

impl Eq for Type {}

/// `TypeArena::add_type<T: Into<Type>>` is designed to accept a bare type
/// variant (mirroring C++ `arena->addType(GenericType{...})`). These `From`
/// impls wrap each variant struct into a non-persistent `Type`, exactly as the
/// C++ `Type(variant)` constructor does.
macro_rules! impl_from_variant_for_type {
    ($variant:ident, $ty:path) => {
        impl From<$ty> for Type {
            fn from(v: $ty) -> Self {
                Type::new(TypeVariant::$variant(v))
            }
        }
    };
}

impl_from_variant_for_type!(Free, crate::records::free_type::FreeType);
impl_from_variant_for_type!(Error, crate::type_aliases::error_type::ErrorType);
impl_from_variant_for_type!(Generic, crate::records::generic_type::GenericType);
impl_from_variant_for_type!(Primitive, crate::records::primitive_type::PrimitiveType);
impl_from_variant_for_type!(Singleton, crate::records::singleton_type::SingletonType);
impl_from_variant_for_type!(Blocked, crate::records::blocked_type::BlockedType);
impl_from_variant_for_type!(
    PendingExpansion,
    crate::records::pending_expansion_type::PendingExpansionType
);
impl_from_variant_for_type!(Function, crate::records::function_type::FunctionType);
impl_from_variant_for_type!(Table, crate::records::table_type::TableType);
impl_from_variant_for_type!(Metatable, crate::records::metatable_type::MetatableType);
impl_from_variant_for_type!(Extern, crate::records::extern_type::ExternType);
impl_from_variant_for_type!(Any, crate::records::any_type::AnyType);
impl_from_variant_for_type!(Union, crate::records::union_type::UnionType);
impl_from_variant_for_type!(
    Intersection,
    crate::records::intersection_type::IntersectionType
);
impl_from_variant_for_type!(Lazy, crate::records::lazy_type::LazyType);
impl_from_variant_for_type!(Unknown, crate::records::unknown_type::UnknownType);
impl_from_variant_for_type!(Never, crate::records::never_type::NeverType);
impl_from_variant_for_type!(Negation, crate::records::negation_type::NegationType);
impl_from_variant_for_type!(NoRefine, crate::records::no_refine_type::NoRefineType);
impl_from_variant_for_type!(
    TypeFunctionInstance,
    crate::records::type_function_instance_type::TypeFunctionInstanceType
);
