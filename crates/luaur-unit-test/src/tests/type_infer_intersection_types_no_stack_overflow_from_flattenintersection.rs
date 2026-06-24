//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_no_stack_overflow_from_flattenintersection() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local l0,l0
        repeat
        type t0 = ((any)|((any)&((any)|((any)&((any)|(any))))))&(t0)
        function _(l0):(t0)&(t0)
            while nil do
            end
        end
        until _(_)(_)._
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty());
}
