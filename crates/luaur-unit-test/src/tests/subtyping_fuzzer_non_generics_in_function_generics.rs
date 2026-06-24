use crate::records::fixture::Fixture;
use alloc::string::String;

#[cfg(test)]
#[test]
fn subtyping_fuzzer_non_generics_in_function_generics() {
    let mut fixture = Fixture::default();

    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local _ = _
        function _(l0)
        for _ in _(_) do
        end
        l0[_](
            _(_()) + _
        )
        end
        _(_)
    "#,
        ),
        None,
    );
}
