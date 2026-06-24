use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get;
use crate::records::metatable_type::MetatableType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::table_type::TableType;

impl Subtyping {
    pub fn is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(
        &mut self,
        env: &mut SubtypingEnvironment,
        sub_mt: &MetatableType,
        super_prim: &PrimitiveType,
        scope: *mut Scope,
    ) -> SubtypingResult {
        if super_prim.r#type == PrimitiveType::Table {
            let followed = unsafe { follow_type_id(sub_mt.table()) };
            if let Some(sub_table) = unsafe { get::<TableType>(followed).as_ref() } {
                return self.is_covariant_with_subtyping_environment_table_type_primitive_type_not_null_scope(env, sub_table, super_prim, scope);
            } else if let Some(sub_nested_mt) = unsafe { get::<MetatableType>(followed).as_ref() } {
                return self.is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(env, sub_nested_mt, super_prim, scope);
            }
        }
        SubtypingResult {
            is_subtype: false,
            ..Default::default()
        }
    }
}
