use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::metatable_type::MetatableType;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_type::TableType;
use luaur_common::FFlag;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_metatable_type_table_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_mt: &MetatableType,
        super_table: &TableType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        let sub_table_id = unsafe { follow_type_id(sub_mt.table()) };
        if let Some(sub_table) = unsafe { get_type_id::<TableType>(sub_table_id).as_ref() } {
            let sub_mt_id = unsafe { follow_type_id(sub_mt.metatable()) };
            if let Some(sub_mt_table) = unsafe { get_type_id::<TableType>(sub_mt_id).as_ref() } {
                if let Some(index_prop) = sub_mt_table.props.get("__index") {
                    if let Some(read_ty) = index_prop.read_ty {
                        let index_table_id = unsafe { follow_type_id(read_ty) };
                        if let Some(index_table) =
                            unsafe { get_type_id::<TableType>(index_table_id).as_ref() }
                        {
                            let mut faux_sub_table = sub_table.clone();
                            for (name, prop) in &index_table.props {
                                if prop.read_ty.is_some()
                                    && !faux_sub_table.props.contains_key(name)
                                {
                                    faux_sub_table.props.insert(
                                        name.clone(),
                                        Property::readonly(prop.read_ty.unwrap()),
                                    );
                                }
                            }
                            return if FFlag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
                                self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(env, &faux_sub_table, super_table, false, scope)
                            } else {
                                self.is_covariant_with_deprecated(
                                    env,
                                    &faux_sub_table,
                                    super_table,
                                    false,
                                    scope,
                                )
                            };
                        }
                    }
                }
            }
            return if FFlag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
                self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(env, sub_table, super_table, false, scope)
            } else {
                self.is_covariant_with_deprecated(env, sub_table, super_table, false, scope)
            };
        }
        SubtypingResult {
            is_subtype: false,
            ..Default::default()
        }
    }
}
