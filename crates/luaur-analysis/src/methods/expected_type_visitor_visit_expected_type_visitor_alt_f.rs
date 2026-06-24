use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use crate::records::function_type::FunctionType;
use luaur_ast::records::ast_expr_call::AstExprCall;

impl ExpectedTypeVisitor {
    pub fn visit_ast_expr_call(&mut self, expr: *mut AstExprCall) -> bool {
        let expr_ref = unsafe { &*expr };

        let ty = unsafe {
            let mut found =
                (*self.ast_overload_resolved_types).find(&(expr_ref as *const _ as *const _));
            if found.is_none() {
                found = (*self.ast_types).find(&(expr_ref.func as *const _));
            }
            found
        };

        if let Some(&ty_id) = ty {
            let followed_ty = unsafe { follow_type_id(ty_id) };
            let ftv = unsafe { get_type_id::<FunctionType>(followed_ty) };

            if !ftv.is_null() {
                let ftv_ref = unsafe { &*ftv };
                let mut it = begin_type_pack_id(ftv_ref.arg_types);
                let end_it = end_type_pack_id(ftv_ref.arg_types);
                let mut idx = 0;

                if expr_ref.self_ && !it.operator_eq(&end_it) {
                    it.operator_inc();
                }

                while idx < expr_ref.args.size && !it.operator_eq(&end_it) {
                    let arg_type = *it.operator_deref();
                    let arg_expr = unsafe { *expr_ref.args.data.add(idx) };
                    self.apply_expected_type(arg_type, arg_expr as *const _);
                    it.operator_inc();
                    idx += 1;
                }
            }
        }

        true
    }
}
