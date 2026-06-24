use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::records::type_attacher::TypeAttacher;
use crate::records::type_rehydration_options::TypeRehydrationOptions;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::synthetic_names::SyntheticNames;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_type_list::AstTypeList;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
use luaur_ast::records::location::Location;

impl TypeAttacher {
    pub fn visit_ast_expr_function(&mut self, fn_: *mut AstExprFunction) -> bool {
        let fn_ref = unsafe { &*fn_ };

        for i in 0..fn_ref.args.size {
            let arg = unsafe { *fn_ref.args.data.add(i) };
            self.visit_local(arg);
        }

        if fn_ref.return_annotation.is_null() {
            // C++ `if (auto result = getScope(fn->body->location))` — Rust
            // `get_scope` always resolves an enclosing scope.
            let body_location = unsafe { (*fn_ref.body).base.base.location };
            let result = self.get_scope(&body_location);
            let ret: TypePackId = result.return_type;
            let (_v, tail) = flatten_type_pack_id(ret);

            let mut variadic_annotation: *mut AstTypePack = core::ptr::null_mut();
            if let Some(tail_tp) = tail {
                let mut rehydrator =
                    TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
                        self.allocator,
                        &mut self.synthetic_names as *mut SyntheticNames,
                        &TypeRehydrationOptions::default(),
                    );
                variadic_annotation = rehydrator.rehydrate(tail_tp);
            }

            let types = self.type_ast_pack(ret);
            let type_list = AstTypeList {
                types,
                tail_type: variadic_annotation,
            };
            let allocator = unsafe { &mut *self.allocator };
            unsafe {
                (*fn_).return_annotation = allocator
                    .alloc(AstTypePackExplicit::new(Location::default(), type_list))
                    as *mut AstTypePack;
            }
        }

        true
    }
}
