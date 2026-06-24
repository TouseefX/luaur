//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:743:type_function_fuzzer_numeric_binop_doesnt_assert_on_generalize_free_type`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - translates_to -> rust_item type_function_fuzzer_numeric_binop_doesnt_assert_on_generalize_free_type

#[cfg(test)]
#[test]
fn type_function_fuzzer_numeric_binop_doesnt_assert_on_generalize_free_type() {
    use crate::records::type_function_fixture::TypeFunctionFixture;
    use alloc::string::String;

    let mut fixture = TypeFunctionFixture::type_function_fixture();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
Module 'l0':
local _ = (67108864)(_ >= _).insert
do end
do end
_(...,_(_,_(_()),_()))
(67108864)()()
_(_ ~= _ // _,l0)(_(_({n0,})),_(_),_)
_(setmetatable(_,{[...]=_,}))

"#,
        ),
        None,
    );
}
