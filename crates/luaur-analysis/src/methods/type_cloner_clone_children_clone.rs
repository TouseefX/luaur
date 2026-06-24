use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::records::type_cloner::TypeCloner;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;

impl TypeCloner {
    /// `void TypeCloner::cloneChildren(TypeId ty)`. Reference: `Clone.cpp:225-234`.
    ///
    /// C++: `visit([&](auto&& t){ return cloneChildren(&t); }, asMutable(ty)->ty);`
    /// — dispatches over the `TypeVariant` to the matching concrete
    /// `cloneChildren(&t)` overload, passing a mutable pointer to the held payload.
    pub fn clone_children_type_id(&mut self, ty: TypeId) {
        let tv: &mut TypeVariant = unsafe { &mut (*as_mutable_type_id(ty)).ty };
        match tv {
            // `Bound<TypeId>` is repr-transparent over its single `TypeId` field,
            // so a pointer to the held `TypeId` is a valid `*mut BoundType`.
            TypeVariant::Bound(inner) => {
                self.clone_children_bound_type(inner as *mut TypeId as *mut BoundType)
            }
            TypeVariant::Error(inner) => self.clone_children_error_type(inner as *mut ErrorType),
            TypeVariant::Free(inner) => self.clone_children_free_type(inner),
            TypeVariant::Generic(inner) => self.clone_children_generic_type(inner),
            TypeVariant::Primitive(inner) => self.clone_children_primitive_type(inner),
            TypeVariant::Singleton(inner) => self.clone_children_singleton_type(inner),
            TypeVariant::Blocked(inner) => self.clone_children_blocked_type(inner),
            TypeVariant::PendingExpansion(inner) => {
                self.clone_children_pending_expansion_type(inner)
            }
            TypeVariant::Function(inner) => self.clone_children_function_type(inner),
            TypeVariant::Table(inner) => self.clone_children_table_type(inner),
            TypeVariant::Metatable(inner) => self.clone_children_metatable_type(inner),
            TypeVariant::Extern(inner) => self.clone_children_extern_type(inner),
            TypeVariant::Any(inner) => self.clone_children_any_type(inner),
            TypeVariant::Union(inner) => self.clone_children_union_type(inner),
            TypeVariant::Intersection(inner) => self.clone_children_intersection_type(inner),
            TypeVariant::Lazy(inner) => self.clone_children_lazy_type(inner),
            TypeVariant::Unknown(inner) => self.clone_children_unknown_type(inner),
            TypeVariant::Never(inner) => self.clone_children_never_type(inner),
            TypeVariant::Negation(inner) => self.clone_children_negation_type(inner),
            TypeVariant::NoRefine(inner) => self.clone_children_no_refine_type(inner),
            TypeVariant::TypeFunctionInstance(inner) => {
                self.clone_children_type_function_instance_type(inner)
            }
        }
    }
}
