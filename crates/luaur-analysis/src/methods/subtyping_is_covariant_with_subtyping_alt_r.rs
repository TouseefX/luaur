use crate::functions::lookup_extern_type_prop::lookup_extern_type_prop;
use crate::records::extern_type::ExternType;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_type_id_extern_type_type_id_table_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_ty: TypeId,
        sub_extern_type: &ExternType,
        super_ty: TypeId,
        super_table: &TableType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let mut result = SubtypingResult {
            is_subtype: true,
            ..Default::default()
        };

        *env.substitutions.get_or_insert(super_ty) = sub_ty;

        for (name, prop) in &super_table.props {
            let class_prop = unsafe { lookup_extern_type_prop(sub_extern_type, name) };
            if !class_prop.is_null() {
                let prop_ref = unsafe { &*class_prop };
                result.and_also(
                    self.is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
                        env,
                        prop_ref,
                        prop,
                        name.as_ref(),
                        false,
                        scope,
                    ),
                    crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy::Any,
                );
            } else {
                result.is_subtype = false;
                break;
            }
        }

        if super_table.indexer.is_some() && sub_extern_type.indexer.is_some() {
            result.and_also(
                self.is_covariant_with_subtyping_environment_table_indexer_table_indexer_not_null_scope(
                    env,
                    sub_extern_type.indexer.as_ref().unwrap(),
                    super_table.indexer.as_ref().unwrap(),
                    scope,
                ),
                crate::enums::subtyping_suppression_policy::SubtypingSuppressionPolicy::Any,
            );
        } else if super_table.indexer.is_some() && sub_extern_type.indexer.is_none() {
            result.is_subtype = false;
        }

        *env.substitutions.get_or_insert(super_ty) = core::ptr::null();

        result
    }
}
