//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1753:type_infer_generics_generalization_no_cyclic_intersections`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_generics_generalization_no_cyclic_intersections

#[cfg(test)]
#[test]
fn type_infer_generics_generalization_no_cyclic_intersections() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local f, t, n = pairs({"foo"})
        local k, v = f(t)
    "#,
        ),
        None,
    );

    assert_eq!(
        "({string}, number?) -> (number?, string)",
        to_string_type_id(fixture.base.require_type_string(&String::from("f")))
    );
    assert_eq!(
        "{string}",
        to_string_type_id(fixture.base.require_type_string(&String::from("t")))
    );
    assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_string(&String::from("k")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("v")))
    );
}
