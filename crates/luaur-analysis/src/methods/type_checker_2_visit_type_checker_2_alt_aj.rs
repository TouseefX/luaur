use crate::enums::type_context::TypeContext;
use crate::enums::value_context::ValueContext;
use crate::functions::match_assert::match_assert;
use crate::functions::match_type_of::match_type_of;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_call::AstExprCall;

impl TypeChecker2 {
    pub fn visit_ast_expr_call(&mut self, call: *mut AstExprCall) {
        let mut flipper: Option<InConditionalContext> = None;

        unsafe {
            // We want to preserve the existing conditional context if we are in a `typeof` call.
            if !match_type_of(&*call) {
                flipper = Some(InConditionalContext::new(
                    &mut self.type_context as *mut TypeContext,
                    TypeContext::Default,
                ));
            }

            self.visit_ast_expr_value_context((*call).func, ValueContext::RValue);

            if match_assert(&*call) && (*call).args.size > 0 {
                {
                    // C++: `InConditionalContext flipper(&typeContext);` (TypeChecker2.cpp:1843)
                    // uses the default `newValue = TypeContext::Condition` so that the first
                    // argument of `assert(...)` is checked in a conditional context (refinements
                    // like `assert(typeof(x) == "table")` apply to property accesses inside it).
                    let _flipper = InConditionalContext::new(
                        &mut self.type_context as *mut TypeContext,
                        TypeContext::Condition,
                    );
                    self.visit_ast_expr_value_context(*(*call).args.data, ValueContext::RValue);
                }

                for i in 1..(*call).args.size {
                    let arg = *(*call).args.data.add(i);
                    self.visit_ast_expr_value_context(arg, ValueContext::RValue);
                }
            } else {
                for i in 0..(*call).args.size {
                    let arg = *(*call).args.data.add(i);
                    self.visit_ast_expr_value_context(arg, ValueContext::RValue);
                }
            }

            self.visit_call(call);
        }
    }
}
