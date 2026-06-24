//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_bounds_propagate_into_free_intersection_bounds() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(
        &FFlag::LuauPropagateFreeTypesIntoUnionAndIntersectionBounds,
        true,
    );
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f<T>(a: T & string): T
            return a
        end

        local b = f("hello")
        local c = f(("world" :: string))
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
}
