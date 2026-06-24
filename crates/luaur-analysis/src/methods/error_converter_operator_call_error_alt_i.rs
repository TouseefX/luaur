use crate::functions::wrong_number_of_args_string::wrong_number_of_args_string;
use crate::records::count_mismatch::{CountMismatch, CountMismatchContext};
use crate::records::error_converter::ErrorConverter;
use alloc::string::String;
use alloc::string::ToString;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ErrorConverter {
    pub fn operator_call_19(&self, e: &CountMismatch) -> String {
        let expected_s = if e.expected == 1 { "" } else { "s" };
        let actual_verb = if e.actual == 1 { "is" } else { "are" };

        match e.context {
            CountMismatchContext::Return => {
                return alloc::format!(
                    "Expected to return {} value{}, but {} {} returned here",
                    e.expected,
                    expected_s,
                    e.actual,
                    actual_verb
                );
            }
            CountMismatchContext::FunctionResult => {
                // It is alright if right hand side produces more values than the
                // left hand side accepts. In this context consider only the opposite case.
                return alloc::format!(
                    "Function only returns {} value{}, but {} {} required here",
                    e.expected,
                    expected_s,
                    e.actual,
                    actual_verb
                );
            }
            CountMismatchContext::ExprListResult => {
                return alloc::format!(
                    "Expression list has {} value{}, but {} {} required here",
                    e.expected,
                    expected_s,
                    e.actual,
                    actual_verb
                );
            }
            CountMismatchContext::Arg => {
                let omit_function_name = e.function.ends_with(".setgenerics");

                if !e.function.is_empty() && !omit_function_name {
                    return alloc::format!(
                        "Argument count mismatch. Function '{}' {}",
                        e.function,
                        wrong_number_of_args_string(
                            e.expected,
                            e.maximum,
                            e.actual,
                            core::ptr::null(),
                            e.is_variadic
                        )
                    );
                } else {
                    return alloc::format!(
                        "Argument count mismatch. Function {}",
                        wrong_number_of_args_string(
                            e.expected,
                            e.maximum,
                            e.actual,
                            core::ptr::null(),
                            e.is_variadic
                        )
                    );
                }
            }
        }

        LUAU_ASSERT!(false);
        String::new()
    }
}
