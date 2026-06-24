use crate::enums::polarity::Polarity;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::scope::Scope;
use crate::records::unknown_symbol::{Context, UnknownSymbol};
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_error_data::IntoTypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;
use luaur_common::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
        &mut self,
        scope: *mut Scope,
        tp: *mut AstTypePack,
        in_type_argument: bool,
        replace_error_with_fresh: bool,
    ) -> TypePackId {
        let mut result: TypePackId = core::ptr::null_mut();

        let node = tp as *mut AstNode;
        let explicit = unsafe { (*node).as_item_mut::<AstTypePackExplicit>() };
        if !explicit.is_null() {
            result = self.resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
                scope,
                &unsafe { &*explicit }.type_list,
                in_type_argument,
                replace_error_with_fresh,
            );
        } else {
            let variadic = unsafe { (*node).as_item_mut::<AstTypePackVariadic>() };
            if !variadic.is_null() {
                let ty: TypeId = self.resolve_type_constraint_generator_alt_b(
                    scope,
                    unsafe { (*variadic).variadic_type },
                    in_type_argument,
                    replace_error_with_fresh,
                );
                result = unsafe {
                    (*self.arena).add_type_pack_t(VariadicTypePack { ty, hidden: false })
                };
            } else {
                let generic = unsafe { (*node).as_item_mut::<AstTypePackGeneric>() };
                if !generic.is_null() {
                    let generic_name_ptr = unsafe { (*generic).generic_name.value };
                    let generic_name_str = unsafe {
                        core::ffi::CStr::from_ptr(generic_name_ptr)
                            .to_string_lossy()
                            .into_owned()
                    };

                    if let Some(lookup) = unsafe { &*scope }.lookup_pack(&generic_name_str) {
                        result = lookup;
                    } else {
                        let error = UnknownSymbol::new(generic_name_str, Context::Type);
                        let location = unsafe { (*tp).base.location };
                        self.report_error(location, error.into_type_error_data());
                        result = unsafe { (*self.builtin_types).errorTypePack };
                    }
                } else {
                    LUAU_ASSERT!(false);
                    result = unsafe { (*self.builtin_types).errorTypePack };
                }
            }
        }

        let followed = unsafe { follow_type_pack_id(result) };
        if let Some(gtp) = unsafe { get_mutable_type_pack_id::<GenericTypePack>(followed).as_mut() }
        {
            gtp.polarity = (gtp.polarity & Polarity::Mixed) | self.polarity;
        }

        if let Some(module) = &self.module {
            let module_ptr =
                alloc::sync::Arc::as_ptr(module) as *mut crate::records::module::Module;
            unsafe {
                *(*module_ptr)
                    .ast_resolved_type_packs
                    .get_or_insert(tp as *const AstTypePack) = result;
            }
        }

        result
    }
}
