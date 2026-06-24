//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_refinement_through_erroring() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        type Payload = { payload: number }

        local function decode(s: string): Payload?
            return (nil :: any)
        end

        local function decodeEx(s: string): Payload
            local p = decode(s)
            if not p then
                error("failed to decode payload!!!")
            end
            return p
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
