//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2458:type_infer_tables_error_detailed_metatable_prop`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record First (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_tables_error_detailed_metatable_prop

#[cfg(test)]
#[test]
fn type_infer_tables_error_detailed_metatable_prop() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::LuauInstantiateInSubtyping, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local a1 = setmetatable({ x = 2, y = 3 }, { __call = function(s) end });
local b1 = setmetatable({ x = 2, y = "hello" }, { __call = function(s) end });
local c1: typeof(a1) = b1

local a2 = setmetatable({ x = 2, y = 3 }, { __call = function(s) end });
local b2 = setmetatable({ x = 2, y = 4 }, { __call = function(s, t) end });
local c2: typeof(a2) = b2
    "#,
        ),
        None,
    );

    let expected1 = r#"Expected this to be 'a1', but got 'b1'
caused by:
  Expected this to be exactly
	'{| x: number, y: number |}'
but got
	'{| x: number, y: string |}'
caused by:
  Property 'y' is not compatible.
Expected this to be exactly 'number', but got 'string'"#;
    let expected2 = r#"Expected this to be 'a2', but got 'b2'
caused by:
  Expected this to be exactly
	'{| __call: <a>(a) -> () |}'
but got
	'{| __call: <a, b>(a, b) -> () |}'
caused by:
  Property '__call' is not compatible.
Expected this to be exactly
	'<a>(a) -> ()'
but got
	'<a, b>(a, b) -> ()'; different number of generic type parameters"#;

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        let expected = "Expected this to be 'a1', but got 'b1'; \nin the table portion, accessing `y` results in `string` in the latter type and `number` in the former type, and `string` is not exactly `number`";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        assert_eq!(expected1, to_string_type_error(&result.errors[0]));
        assert_eq!(expected2, to_string_type_error(&result.errors[1]));
    }
}
