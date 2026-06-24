//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2352:type_infer_is_safe_integer_example`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - calls -> function isInteger (Analysis/src/Type.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_is_safe_integer_example

#[cfg(test)]
#[test]
fn type_infer_is_safe_integer_example() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/isInteger"),
        String::from(
            r#"
        --!strict
        return function(value)
            return type(value) == "number" and value ~= math.huge and value == math.floor(value)
        end
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/MAX_SAFE_INTEGER"),
        String::from(
            r#"
        --!strict
        return 42
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/Util"),
        String::from(
            r#"
        --!strict
        local isInteger = require(script.Parent.isInteger)
        local MAX_SAFE_INTEGER = require(script.Parent.MAX_SAFE_INTEGER)
        return function(value)
        	return isInteger(value) and math.abs(value) <= MAX_SAFE_INTEGER
        end
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Util"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
