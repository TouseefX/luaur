use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::free_type::FreeType;
use crate::records::occurs_check_failed::OccursCheckFailed;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;

impl TypeChecker {
    pub fn check_block_type_aliases(
        &mut self,
        scope: &ScopePtr,
        sorted: &mut alloc::vec::Vec<*mut AstStat>,
    ) {
        for stat in sorted.iter() {
            let stat_ptr = *stat;
            let typealias = unsafe {
                crate::rtti::ast_node_as::<AstStatTypeAlias>(
                    stat_ptr as *mut crate::records::ast_node::AstNode,
                )
            };
            if typealias.is_null() {
                continue;
            }

            let typealias_ref = unsafe { &*typealias };

            let alias_name = unsafe { core::ffi::CStr::from_ptr(typealias_ref.name.value) };
            if alias_name.to_bytes() == b"%error-id%" || alias_name.to_bytes() == b"typeof" {
                continue;
            }

            let bindings = if typealias_ref.exported {
                &scope.exported_type_bindings
            } else {
                &scope.private_type_bindings
            };

            let name: Name = alias_name.to_string_lossy().into_owned();

            if self
                .duplicate_type_aliases
                .contains(&(typealias_ref.exported, name.clone()))
            {
                continue;
            }

            let type_binding = bindings.get(&name);
            if type_binding.is_none() {
                continue;
            }

            let type_id = unsafe { follow_type_id(type_binding.unwrap().r#type()) };

            if unsafe { !get_type_id::<FreeType>(type_id).is_null() } {
                unsafe {
                    (*as_mutable_type_id(type_id)).ty =
                        TypeVariant::Bound(self.error_recovery_type_type_id(self.any_type));
                }

                let error_data = TypeErrorData::OccursCheckFailed(OccursCheckFailed::default());
                let error = TypeError::type_error_location_type_error_data(
                    typealias_ref.base.base.location,
                    error_data,
                );
                self.report_error_type_error(&error);
            }
        }
    }
}
