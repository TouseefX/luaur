//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1018:type_infer_provisional_luau_polyfill_map_entries`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_provisional_luau_polyfill_map_entries

#[cfg(test)]
#[test]
fn type_infer_provisional_luau_polyfill_map_entries() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("Module/Map"),
        String::from(
            r#"
--!strict

type Object = { [any]: any }
type Array<T> = { [number]: T }
type Table<T, V> = { [T]: V }
type Tuple<T, V> = Array<T | V>

local Map = {}

export type Map<K, V> = {
	size: number,
	-- method definitions
	set: (self: Map<K, V>, K, V) -> Map<K, V>,
	get: (self: Map<K, V>, K) -> V | nil,
	clear: (self: Map<K, V>) -> (),
	delete: (self: Map<K, V>, K) -> boolean,
	has: (self: Map<K, V>, K) -> boolean,
	keys: (self: Map<K, V>) -> Array<K>,
	values: (self: Map<K, V>) -> Array<V>,
	entries: (self: Map<K, V>) -> Array<Tuple<K, V>>,
	ipairs: (self: Map<K, V>) -> any,
	[K]: V,
	_map: { [K]: V },
	_array: { [number]: K },
}

function Map:entries()
	return {}
end

local function coerceToTable(mapLike: Map<any, any> | Table<any, any>): Array<Tuple<any, any>>
    local e = mapLike:entries();
    return e
end

    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/Map"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
