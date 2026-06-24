//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:90:type_function_function_as_fn_ret`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_function_as_fn_ret

#[cfg(test)]
#[test]
fn type_function_function_as_fn_ret() {
    use crate::records::type_function_fixture::TypeFunctionFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = TypeFunctionFixture::type_function_fixture();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local swapper: <T>(T) -> Swap<T>
        local a = swapper(123)
        local b = swapper("foo")
        local c = swapper(false)
    "#,
        ),
        None,
    );

    let a_ty = to_string_type_id(fixture.base.require_type_string(&String::from("a")));
    let b_ty = to_string_type_id(fixture.base.require_type_string(&String::from("b")));
    let c_ty = to_string_type_id(fixture.base.require_type_string(&String::from("c")));

    assert_eq!(
        1,
        result.errors.len(),
        "a={a_ty}, b={b_ty}, c={c_ty}, errors={:?}",
        result.errors
    );
    assert_eq!("string", a_ty);
    assert_eq!("number", b_ty);
    assert_eq!("Swap<boolean>", c_ty);
    assert_eq!(
        "Type function instance Swap<boolean> is uninhabited",
        to_string_type_error(&result.errors[0])
    );
}
