use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_subtype_type_id_type_id_not_null_scope(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let mut env = crate::records::subtyping_environment::SubtypingEnvironment {
            parent: core::ptr::null_mut(),
            mapped_generics: luaur_common::records::dense_hash_map::DenseHashMap::new(
                core::ptr::null(),
            ),
            mapped_generic_packs:
                crate::records::mapped_generic_environment::MappedGenericEnvironment {
                    frames: alloc::vec::Vec::new(),
                    current_scope_index: None,
                },
            substitutions: luaur_common::records::dense_hash_map::DenseHashMap::new(
                core::ptr::null(),
            ),
            seen_set_cache: luaur_common::records::dense_hash_map::DenseHashMap::new((
                core::ptr::null(),
                core::ptr::null(),
            )),
            iteration_count: 0,
        };

        let mut result = self
            .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                &mut env, sub_ty, super_ty, scope,
            );

        if result.normalization_too_complex {
            if result.is_cacheable {
                self.result_cache
                    .try_insert((sub_ty, super_ty), result.clone());
            }
            return result;
        }

        if result.is_cacheable {
            self.result_cache
                .try_insert((sub_ty, super_ty), result.clone());
        }

        result
    }
}
