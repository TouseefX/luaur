//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1403:type_function_fuzz_len_type_function_follow`
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
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - translates_to -> rust_item type_function_fuzz_len_type_function_follow

#[cfg(test)]
#[test]
fn type_function_fuzz_len_type_function_follow() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::default();
    let _ = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local _
        _ = true
        for l0=_,_,# _ do
        end
        for l0=_,_ do
        if _ then
        _ += _
        end
        end
    "#,
        ),
        None,
    );
}
