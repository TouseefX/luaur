use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_mismatch::TypeMismatch;
use crate::type_aliases::singleton_variant::SingletonVariant;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;

impl TypeChecker2 {
    pub fn visit_ast_expr_constant_string(&mut self, expr: *mut AstExprConstantString) {
        // strings use specialized inference logic for singleton typeArguments,
        // which can lead to real type errors here.
        unsafe {
            let string_value = (*expr).value;
            let len = string_value.size as usize;
            let data_ptr = string_value.data;

            let string_bytes = core::slice::from_raw_parts(data_ptr as *const u8, len);
            let string_data = alloc::string::String::from_utf8_lossy(string_bytes).into_owned();

            // C++: module->internalTypes.addType(SingletonType{StringSingleton{...}})
            let best_type = (*self.module)
                .internal_types
                .add_type(SingletonType::singleton_type(SingletonVariant::V1(
                    StringSingleton::new(string_data),
                )));
            let inferred_type =
                self.lookup_type(expr as *mut luaur_ast::records::ast_expr::AstExpr);
            let scope = self.find_innermost_scope((*expr).base.base.location);

            let mut r = (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
                best_type,
                inferred_type,
                scope,
            );

            if !self
                .is_error_suppressing_location_type_id((*expr).base.base.location, inferred_type)
            {
                if !r.is_subtype {
                    self.report_error_type_error_data_location(
                        TypeMismatch::from_wanted_given(inferred_type, best_type).into(),
                        &(*expr).base.base.location,
                    );
                }

                for e in &mut r.errors {
                    e.location = (*expr).base.base.location;
                }
                self.report_errors(core::mem::take(&mut r.errors));
            }
        }
    }
}
