//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_propagates_name() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from(
        r#"
        type A={a:number}
        type B={b:string}

        local function f(t: A & B)
            return t
        end
    "#,
    );

    let expected = String::from(
        r#"
        type A={a:number}
        type B={b:string}

        local function f(t: A & B): A&B
            return t
        end
    "#,
    );

    assert_eq!(expected, fixture.decorate_with_types(&code));
}
