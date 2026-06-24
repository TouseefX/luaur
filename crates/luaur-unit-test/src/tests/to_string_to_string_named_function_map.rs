//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_named_function_map() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_named_function_to_string::to_string_named_function_string_function_type;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function map(arr, fn)
            local t = {}
            for i = 0, #arr do
                t[i] = fn(arr[i])
            end
            return t
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("map"));
    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(ty)).as_ref() }
        .expect("expected map to be a function type");
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "map<a, b>(arr: {a}, fn: (a) -> (b, ...unknown)): {b}",
            to_string_named_function_string_function_type("map", ftv)
        );
    } else {
        assert_eq!(
            "map<a, b>(arr: {a}, fn: (a) -> b): {b}",
            to_string_named_function_string_function_type("map", ftv)
        );
    }
}
