//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_initializers_are_checked_against_annotations() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from("local a: number = \"Hello Types!\""),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}
