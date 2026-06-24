use crate::enums::polarity::Polarity;
use crate::functions::invert_polarity::invert_polarity;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::function_argument::FunctionArgument;
use crate::records::function_type::FunctionType;
use crate::records::r#type::Type;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ffi::CStr;
use luaur_ast::records::ast_attr::AstAttr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_function::AstTypeFunction;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn resolve_function_type(
        &mut self,
        scope: &ScopePtr,
        _ty: *mut AstType,
        fn_node: &AstTypeFunction,
        in_type_arguments: bool,
        replace_error_with_fresh: bool,
    ) -> TypeId {
        let has_generics = fn_node.generics.size > 0 || fn_node.generic_packs.size > 0;
        let mut signature_scope = scope.clone();

        let mut generic_types = Vec::new();
        let mut generic_type_packs = Vec::new();

        if has_generics {
            signature_scope =
                self.child_scope(&fn_node.base.base as *const AstNode as *mut AstNode, scope);

            let generic_definitions =
                self.create_generics(&signature_scope, fn_node.generics, false, true);
            let generic_pack_definitions =
                self.create_generic_packs(&signature_scope, fn_node.generic_packs, false, true);

            for (_, g) in generic_definitions {
                generic_types.push(g.ty);
            }

            for (_, g) in generic_pack_definitions {
                generic_type_packs.push(g.tp);
            }
        }

        let temp_arg_types =
            AstTypePackExplicit::new(Location::default(), fn_node.arg_types.clone());

        let p = self.polarity;
        self.polarity = invert_polarity(self.polarity);
        let arg_types = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
            signature_scope.as_ref() as *const Scope as *mut Scope,
            &temp_arg_types as *const AstTypePackExplicit
                as *mut luaur_ast::records::ast_type_pack::AstTypePack,
            in_type_arguments,
            replace_error_with_fresh,
        );
        self.polarity = p;

        let return_types = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
            signature_scope.as_ref() as *const Scope as *mut Scope,
            fn_node.return_types,
            in_type_arguments,
            replace_error_with_fresh,
        );

        let mut ftv = FunctionType {
            definition: None,
            generics: generic_types,
            generic_packs: generic_type_packs,
            arg_names: Vec::with_capacity(fn_node.arg_names.size as usize),
            tags: Default::default(),
            level: TypeLevel::new(0, 0),
            arg_types,
            ret_types: return_types,
            magic: None,
            has_self: false,
            has_no_free_or_generic_types: false,
            is_checked_function: fn_node.is_checked_function(),
            is_deprecated_function: false,
            deprecated_info: None,
        };

        let deprecated_attr =
            fn_node.get_attribute(luaur_ast::records::ast_attr::AstAttrType::Deprecated);
        if !deprecated_attr.is_null() {
            ftv.is_deprecated_function = true;
            ftv.deprecated_info = Some(Arc::new(unsafe { (*deprecated_attr).deprecated_info() }));
        }

        for i in 0..fn_node.arg_names.size {
            let el = unsafe { *fn_node.arg_names.data.add(i as usize) };
            if let Some(arg) = el {
                ftv.arg_names.push(Some(FunctionArgument {
                    name: unsafe { CStr::from_ptr(arg.0.value).to_string_lossy().into_owned() },
                    location: arg.1,
                }));
            } else {
                ftv.arg_names.push(None);
            }
        }

        unsafe {
            (*self.arena).add_type(Type::new(
                crate::type_aliases::type_variant::TypeVariant::Function(ftv),
            ))
        }
    }
}
