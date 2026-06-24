//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1167:normalize_normalizer_should_be_able_to_detect_cyclic_tables_and_not_stack_overflow`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item normalize_normalizer_should_be_able_to_detect_cyclic_tables_and_not_stack_overflow

#[cfg(test)]
#[test]
fn normalize_normalizer_should_be_able_to_detect_cyclic_tables_and_not_stack_overflow() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_common::{FFlag, FInt};

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let _sfi = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 0);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict

type Array<T> = { [number] : T}
type Object = { [number] : any}

type Set<T> = typeof(setmetatable(
	{} :: {
		size: number,
		-- method definitions
		add: (self: Set<T>, T) -> Set<T>,
		clear: (self: Set<T>) -> (),
		delete: (self: Set<T>, T) -> boolean,
		has: (self: Set<T>, T) -> boolean,
		ipairs: (self: Set<T>) -> any,
	},
	{} :: {
		__index: Set<T>,
		__iter: (self: Set<T>) -> (<K, V>({ [K]: V }, K?) -> (K, V), T),
	}
))

type Map<K, V> = typeof(setmetatable(
	{} :: {
		size: number,
		-- method definitions
		set: (self: Map<K, V>, K, V) -> Map<K, V>,
		get: (self: Map<K, V>, K) -> V | nil,
		clear: (self: Map<K, V>) -> (),
		delete: (self: Map<K, V>, K) -> boolean,
		[K]: V,
		has: (self: Map<K, V>, K) -> boolean,
		keys: (self: Map<K, V>) -> Array<K>,
		values: (self: Map<K, V>) -> Array<V>,
		entries: (self: Map<K, V>) -> Array<Tuple<K, V>>,
		ipairs: (self: Map<K, V>) -> any,
		_map: { [K]: V },
		_array: { [number]: K },
		__index: (self: Map<K, V>, key: K) -> V,
		__iter: (self: Map<K, V>) -> (<K, V>({ [K]: V }, K?) -> (K?, V), V),
		__newindex: (self: Map<K, V>, key: K, value: V) -> (),
	},
	{} :: {
		__index: Map<K, V>,
		__iter: (self: Map<K, V>) -> (<K, V>({ [K]: V }, K?) -> (K, V), V),
		__newindex: (self: Map<K, V>, key: K, value: V) -> (),
	}
))
type mapFn<T, U> = (element: T, index: number) -> U
type mapFnWithThisArg<T, U> = (thisArg: any, element: T, index: number) -> U

function fromSet<T, U>(
	value: Set<T>,
	mapFn: (mapFn<T, U> | mapFnWithThisArg<T, U>)?,
	thisArg: Object?
	-- FIXME Luau: need overloading so the return type on this is more sane and doesn't require manual casts
): Array<U> | Array<T> | Array<string>

    local array : { [number] : string} = {"foo"}
	return array
end

function instanceof(tbl: any, class: any): boolean
    return true
end

function fromArray<T, U>(
	value: Array<T>,
	mapFn: (mapFn<T, U> | mapFnWithThisArg<T, U>)?,
	thisArg: Object?
	-- FIXME Luau: need overloading so the return type on this is more sane and doesn't require manual casts
): Array<U> | Array<T> | Array<string>
	local array : {[number] : string} = {}
	return array
end

return function<T, U>(
	value: string | Array<T> | Set<T> | Map<any, any>,
	mapFn: (mapFn<T, U> | mapFnWithThisArg<T, U>)?,
	thisArg: Object?
	-- FIXME Luau: need overloading so the return type on this is more sane and doesn't require manual casts
): Array<U> | Array<T> | Array<string>
	if value == nil then
		error("cannot create array from a nil value")
	end
	local array: Array<U> | Array<T> | Array<string>

    if instanceof(value, Set) then
		array = fromSet(value :: Set<T>, mapFn, thisArg)
	else
		array = {}
	end


	return array
end
"#,
        ),
        None,
    );
}
