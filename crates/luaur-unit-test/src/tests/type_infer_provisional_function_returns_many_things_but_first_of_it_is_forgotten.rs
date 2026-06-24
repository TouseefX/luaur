//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:452:type_infer_provisional_function_returns_many_things_but_first_of_it_is_forgotten`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_provisional_function_returns_many_things_but_first_of_it_is_forgotten

#[cfg(test)]
#[test]
fn type_infer_provisional_function_returns_many_things_but_first_of_it_is_forgotten() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(): (number, string, boolean)
            if math.random() > 0.5 then
                return 5, "hello", true
            else
                error("something")
            end
        end

        local ok, res, s, b = pcall(f)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.base.require_type_string(&String::from("ok")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("res")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("s")))
    );
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
}
