//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:135:type_infer_provisional_xpcall_returns_what_f_returns`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method Fixture::decorateWithTypes (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_provisional_xpcall_returns_what_f_returns

#[cfg(test)]
#[test]
fn type_infer_provisional_xpcall_returns_what_f_returns() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let code = String::from(
        r#"
        local a, b, c = xpcall(function() return 1, "foo" end, function() return "foo", 1 end)
    "#,
    );

    let expected = r#"
        local a:boolean,b:number,c:string=xpcall(function(): (number,string)return 1,'foo'end,function(): (string,number)return'foo',1 end)
    "#;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
        .base
        .check_string_optional_frontend_options(&code, None);

    assert_eq!(
        "boolean",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    assert_eq!(expected, fixture.base.decorate_with_types(&code));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
