use crate::functions::emit_warning::emit_warning;
use crate::records::lint_integer_parsing::LintIntegerParsing;
use luaur_ast::enums::constant_number_parse_result::ConstantNumberParseResult;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_config::enums::code::Code;

pub fn lint_integer_parsing_visit(
    this: &mut LintIntegerParsing,
    node: *mut AstExprConstantNumber,
) -> bool {
    match unsafe { &(*node).parse_result } {
        ConstantNumberParseResult::Ok | ConstantNumberParseResult::Malformed => {}
        ConstantNumberParseResult::Imprecise => {
            emit_warning(
                    unsafe { &mut *this.context },
                    Code::Code_IntegerParsing,
                    unsafe { (*node).base.base.location },
                    format_args!(
                        "Number literal exceeded available precision and was truncated to closest representable number"
                    ),
                );
        }
        ConstantNumberParseResult::BinOverflow => {
            emit_warning(
                unsafe { &mut *this.context },
                Code::Code_IntegerParsing,
                unsafe { (*node).base.base.location },
                format_args!(
                    "Binary number literal exceeded available precision and was truncated to 2^64"
                ),
            );
        }
        ConstantNumberParseResult::HexOverflow => {
            emit_warning(
                    unsafe { &mut *this.context },
                    Code::Code_IntegerParsing,
                    unsafe { (*node).base.base.location },
                    format_args!(
                        "Hexadecimal number literal exceeded available precision and was truncated to 2^64"
                    ),
                );
        }
        ConstantNumberParseResult::IntOverflow => {
            emit_warning(
                unsafe { &mut *this.context },
                Code::Code_IntegerParsing,
                unsafe { (*node).base.base.location },
                format_args!("Integer number literal was clamped because it was out of range"),
            );
        }
    }

    true
}
