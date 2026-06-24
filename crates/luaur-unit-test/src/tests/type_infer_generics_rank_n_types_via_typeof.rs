#[cfg(test)]
#[test]
fn type_infer_generics_rank_n_types_via_typeof() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local function id(x) return x end
        local x: string = id("hi")
        local y: number = id(37)
        -- This allows <a>(a)->a to be expressed without generic function syntax
        type Id = typeof(id)
        -- The rank 1 restriction causes this not to typecheck, since it's
        -- declared as returning a polytype.
        local function returnsId(): Id
          return id
        end
        -- So this won't typecheck
        local f: Id = returnsId()
        local a: string = f("hi")
        local b: number = f(37)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
