//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1394:type_infer_operators_reworked_or`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_operators_reworked_or

#[cfg(test)]
#[test]
fn type_infer_operators_reworked_or() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local a: number | false = 5
local b: number? = 6
local c: boolean = true
local d: true = true
local e: false = false
local f: nil = false

local a1 = a or 'a'
local b1 = b or 4
local c1 = c or 'c'
local d1 = d or 'd'
local e1 = e or 'e'
local f1 = f or 'f'
    "#,
        ),
        None,
    );

    assert_eq!(
        "number | string",
        to_string_type_id(fixture.base.require_type_string(&String::from("a1")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("b1")))
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "string | true",
            to_string_type_id(fixture.base.require_type_string(&String::from("c1")))
        );
        assert_eq!(
            "string | true",
            to_string_type_id(fixture.base.require_type_string(&String::from("d1")))
        );
    } else {
        assert_eq!(
            "boolean | string",
            to_string_type_id(fixture.base.require_type_string(&String::from("c1")))
        );
        assert_eq!(
            "boolean | string",
            to_string_type_id(fixture.base.require_type_string(&String::from("d1")))
        );
    }
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("e1")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("f1")))
    );
}
