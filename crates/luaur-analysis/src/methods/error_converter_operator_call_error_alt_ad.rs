use crate::enums::op_kind::OpKind;
use crate::records::cannot_infer_binary_operation::CannotInferBinaryOperation;
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;
use luaur_ast::functions::to_string_ast_alt_b::to_string_ast_expr_binary_op;

impl ErrorConverter {
    pub fn operator_call_16(&self, e: &CannotInferBinaryOperation) -> String {
        let mut result = String::from("Unknown type used in ");
        result.push_str(&to_string_ast_expr_binary_op(e.op()));

        match e.kind() {
            OpKind::Comparison => {
                result.push_str(" comparison");
            }
            OpKind::Operation => {
                result.push_str(" operation");
            }
        }

        if let Some(suggested) = e.suggestedToAnnotate() {
            result.push_str("; consider adding a type annotation to '");
            result.push_str(suggested);
            result.push('\'');
        }

        result
    }
}
