//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_for_loop_counter_annotation_is_checked() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(" for i: string = 0, 10 do end "),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}
