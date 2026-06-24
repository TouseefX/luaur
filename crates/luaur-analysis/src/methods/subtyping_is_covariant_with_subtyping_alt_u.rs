use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
use crate::functions::get_type_alt_j::get_type_id;
use crate::methods::path_builder_build::PathBuilderBuild;
use crate::methods::path_builder_mt::PathBuilderMt;
use crate::records::path_builder::PathBuilder;
use crate::records::primitive_type::PrimitiveType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_type::TableType;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_primitive_type_table_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_prim: &PrimitiveType,
        super_table: &TableType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let mut result = SubtypingResult {
            is_subtype: false,
            ..Default::default()
        };

        if sub_prim.r#type == PrimitiveType::String {
            if let Some(metatable) = unsafe {
                get_metatable_type_id_not_null_builtin_types(
                    (*self.builtin_types).stringType,
                    &*self.builtin_types,
                )
            } {
                if let Some(mttv) =
                    unsafe { get_type_id::<TableType>(follow_type_id(metatable)).as_ref() }
                {
                    if let Some(it) = mttv.props.get("__index") {
                        // the `string` metatable should not have any write-only types.
                        LUAU_ASSERT!(!it.read_ty.unwrap().is_null());

                        if let Some(string_table) =
                            unsafe { get_type_id::<TableType>(it.read_ty.unwrap()).as_ref() }
                        {
                            if FFlag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
                                let mut sub_result = self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
                                    env, string_table, super_table, false, scope,
                                );
                                sub_result.with_sub_path(
                                    PathBuilder {
                                        components: alloc::vec::Vec::new(),
                                    }
                                    .mt()
                                    .read_prop("__index")
                                    .build(),
                                );
                                result.or_else(sub_result);
                            } else {
                                let mut sub_result = self.is_covariant_with_deprecated(
                                    env,
                                    string_table,
                                    super_table,
                                    false,
                                    scope,
                                );
                                sub_result.with_sub_path(
                                    PathBuilder {
                                        components: alloc::vec::Vec::new(),
                                    }
                                    .mt()
                                    .read_prop("__index")
                                    .build(),
                                );
                                result.or_else(sub_result);
                            }
                        }
                    }
                }
            }
        } else if sub_prim.r#type == PrimitiveType::Table {
            let is_subtype = super_table.props.is_empty()
                && (super_table.indexer.is_none() || super_table.state == TableState::Generic);
            result.is_subtype = is_subtype;
            return result;
        }

        result
    }
}
