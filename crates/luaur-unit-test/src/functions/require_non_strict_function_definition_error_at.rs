use crate::functions::type_error_data_ref::type_error_data_ref;
use luaur_analysis::records::check_result::CheckResult;
use luaur_analysis::records::non_strict_function_definition_error::NonStrictFunctionDefinitionError;
use luaur_ast::records::position::Position;

pub fn require_non_strict_function_definition_error_at(
    result: &CheckResult,
    position: Position,
    argument: &str,
) {
    for error in &result.errors {
        if error.location.begin == position {
            let Some(error) = type_error_data_ref::<NonStrictFunctionDefinitionError>(error) else {
                panic!(
                    "expected NonStrictFunctionDefinitionError at {:?}",
                    position
                );
            };

            assert_eq!(argument, error.argument());
            return;
        }
    }

    panic!("expected error at {:?}", position);
}
