//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_cli_38393_recursive_intersection_oom() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function _(l0:(t0)&((t0)&(((t0)&((t0)->()))->(typeof(_),typeof(# _)))),l39,...):any
        end
        type t0<t0> = ((typeof(_))&((t0)&(((typeof(_))&(t0))->typeof(_))),{n163:any,})->(any,typeof(_))
        _(_)
    "#,
        ),
        None,
    );
}
