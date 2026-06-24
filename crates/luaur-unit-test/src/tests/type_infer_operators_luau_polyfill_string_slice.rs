//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:1507:type_infer_operators_luau_polyfill_string_slice`
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
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Position Lexer::position (Ast/src/Lexer.cpp)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> macro tostring (VM/src/lvm.h)
//!   - calls -> macro tonumber (VM/src/lvm.h)
//!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item type_infer_operators_luau_polyfill_string_slice

#[cfg(test)]
#[test]
fn type_infer_operators_luau_polyfill_string_slice() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local function slice(str: string, startIndexStr: string | number, lastIndexStr: (string | number)?): string
	local strLen, invalidBytePosition = utf8.len(str)
	assert(strLen ~= nil, ("string `%s` has an invalid byte at position %s"):format(str, tostring(invalidBytePosition)))
    local startIndex = tonumber(startIndexStr)


	-- if no last index length set, go to str length + 1
	local lastIndex = strLen + 1

	assert(typeof(lastIndex) == "number", "lastIndexStr should convert to number")

	if lastIndex > strLen then
		lastIndex = strLen + 1
	end

	local startIndexByte = utf8.offset(str, startIndex)

	return string.sub(str, startIndexByte, startIndexByte)
end

return slice


    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
