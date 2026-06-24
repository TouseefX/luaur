//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_for_loop_counter_annotation() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(" for i: number = 0, 50 do end "),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
