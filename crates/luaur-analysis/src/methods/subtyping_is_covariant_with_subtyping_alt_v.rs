use crate::functions::follow_type::follow_type_id;
use crate::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::methods::path_builder_build::PathBuilderBuild;
use crate::methods::path_builder_mt::PathBuilderMt;
use crate::records::path_builder::PathBuilder;
use crate::records::scope::Scope;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_type::TableType;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_singleton_type_table_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_singleton: &SingletonType,
        super_table: &TableType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let mut result = SubtypingResult {
            is_subtype: false,
            ..Default::default()
        };

        if !get_singleton_type::<StringSingleton>(sub_singleton as *const SingletonType).is_null() {
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
                                let mut pb = PathBuilder {
                                    components: alloc::vec::Vec::new(),
                                };
                                pb.mt();
                                pb.read_prop("__index");
                                sub_result.with_sub_path(pb.build());
                                result.or_else(sub_result);
                            } else {
                                let mut sub_result = self.is_covariant_with_deprecated(
                                    env,
                                    string_table,
                                    super_table,
                                    false,
                                    scope,
                                );
                                let mut pb = PathBuilder {
                                    components: alloc::vec::Vec::new(),
                                };
                                pb.mt();
                                pb.read_prop("__index");
                                sub_result.with_sub_path(pb.build());
                                result.or_else(sub_result);
                            }
                        }
                    }
                }
            }
        }

        result
    }
}
