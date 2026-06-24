//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:96:type_infer_provisional_luau_polyfill_array_filter`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_provisional_luau_polyfill_array_filter

#[cfg(test)]
#[test]
fn type_infer_provisional_luau_polyfill_array_filter() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
-- Implements Javascript's `Array.prototype.filter` as defined below
-- https://developer.cmozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter
type Array<T> = { [number]: T }
type callbackFn<T> = (element: T, index: number, array: Array<T>) -> boolean
type callbackFnWithThisArg<T, U> = (thisArg: U, element: T, index: number, array: Array<T>) -> boolean
type Object = { [string]: any }
return function<T, U>(t: Array<T>, callback: callbackFn<T> | callbackFnWithThisArg<T, U>, thisArg: U?): Array<T>

	local len = #t
	local res = {}
	if thisArg == nil then
		for i = 1, len do
			local kValue = t[i]
			if kValue ~= nil then
				if (callback :: callbackFn<T>)(kValue, i, t) then
					res[i] = kValue
				end
			end
		end
	else
	end

	return res
end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
