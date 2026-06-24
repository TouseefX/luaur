//! Faithful port of `Subtyping::isSubtype(subTp, superTp, scope, bindableGenerics)`
//! — the 4-arg pack overload (Analysis/src/Subtyping.cpp:635-639) that constructs
//! an empty `bindableGenericPacks` and delegates to the 5-arg overload.
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl Subtyping {
    /// C++:
    /// ```cpp
    /// SubtypingResult Subtyping::isSubtype(TypePackId subTp, TypePackId superTp, NotNull<Scope> scope,
    ///     const std::vector<TypeId>& bindableGenerics)
    /// {
    ///     const std::vector<TypePackId> bindableGenericPacks;
    ///     return isSubtype(subTp, superTp, scope, bindableGenerics, bindableGenericPacks);
    /// }
    /// ```
    pub fn is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id(
        &mut self,
        sub_tp: TypePackId,
        super_tp: TypePackId,
        scope: *mut Scope,
        bindable_generics: &Vec<TypeId>,
    ) -> SubtypingResult {
        let bindable_generic_packs: alloc::vec::Vec<TypePackId> = alloc::vec::Vec::new();
        self.is_subtype_type_pack_id_type_pack_id_not_null_scope_vector_type_id_vector_type_pack_id(
            sub_tp,
            super_tp,
            scope,
            bindable_generics,
            &bindable_generic_packs,
        )
    }
}
