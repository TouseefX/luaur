//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_invalidate_type_refinements_upon_assignments() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Ok<T> = { tag: "ok", val: T }
        type Err<E> = { tag: "err", err: E }
        type Result<T, E> = Ok<T> | Err<E>

        local function f<T, E>(res: Result<T, E>)
            assert(res.tag == "ok")
            local tag: "ok", val: T = res.tag, res.val
            res = { tag = "err" :: "err", err = (5 :: any) :: E }
            local tag: "err", err: E = res.tag, res.err
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
