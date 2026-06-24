//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_function_type_with_argument_names_and_self() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local tbl = {}
tbl.a = 2
function tbl:foo(b: number, c: number) return (self.a :: number) + b + c end
type Table = typeof(tbl)
type Foo = typeof(tbl.foo)
local u: Foo
"#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::default();
    opts.function_type_arguments = true;
    let _ = to_string_type_id_to_string_options(
        fixture.require_type_string(&String::from("u")),
        &mut opts,
    );
}
