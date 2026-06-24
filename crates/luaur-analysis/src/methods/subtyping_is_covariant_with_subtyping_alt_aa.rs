use crate::records::normalized_extern_type::NormalizedExternType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_ids::TypeIds;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_normalized_extern_type_type_ids_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_extern_type: &NormalizedExternType,
        super_tables: &TypeIds,
        scope: *mut Scope,
    ) -> SubtypingResult {
        for (sub_extern_type_ty, _) in &sub_extern_type.extern_types {
            let mut result = SubtypingResult::default();

            for super_table_ty in &super_tables.order {
                result.or_else(
                    self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                        env,
                        *sub_extern_type_ty,
                        *super_table_ty,
                        scope,
                    ),
                );
            }

            if !result.is_subtype {
                return result;
            }
        }

        SubtypingResult {
            is_subtype: true,
            ..Default::default()
        }
    }
}
