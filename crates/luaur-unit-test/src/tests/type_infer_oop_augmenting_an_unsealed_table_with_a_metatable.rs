//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:387:type_infer_oop_augmenting_an_unsealed_table_with_a_metatable`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_oop_augmenting_an_unsealed_table_with_a_metatable

#[cfg(test)]
#[test]
fn type_infer_oop_augmenting_an_unsealed_table_with_a_metatable() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local A = {number = 8}

        local B = setmetatable({}, A)

        function B:method()
            return "hello!!"
        end
    "#,
        ),
        None,
    );

    let mut opts = ToStringOptions::to_string_options(true);
    let b_type = fixture.base.require_type_string(&String::from("B"));
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "{ @metatable { number: number }, { method: (unknown) -> string } }"
    } else {
        "{ @metatable {| number: number |}, {| method: <a>(a) -> string |} }"
    };
    assert_eq!(
        expected,
        to_string_type_id_to_string_options(b_type, &mut opts)
    );
}
