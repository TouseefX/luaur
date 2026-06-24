//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_intersection_of_tables_with_top_properties() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x : { p : number?, q : any } & { p : unknown, q : string? })
            local y : { p : number?, q : string? } = x -- OK
            local z : { p : string?, q : number? } = x -- Not OK
        end
    "#,
        ),
        None,
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        concat!(
            "Expected this to be\n",
            "\t'{ p: string?, q: number? }'\n",
            "but got\n",
            "\t'{ p: number?, q: any } & { p: unknown, q: string? }'; \n",
            "this is because \n",
            "\t * in the 1st component of the intersection, accessing `p` has the 1st component of the union as `number` and accessing `p` has the 1st component of the union as `string`, and `number` is not exactly `string`\n",
            "\t * in the 1st component of the intersection, accessing `p` has the 1st component of the union as `number` and accessing `p` has the 2nd component of the union as `nil`, and `number` is not exactly `nil`\n",
            "\t * in the 1st component of the intersection, accessing `p` has the 2nd component of the union as `nil` and accessing `p` has the 1st component of the union as `string`, and `nil` is not exactly `string`\n",
            "\t * in the 1st component of the intersection, accessing `q` results in `any` and accessing `q` has the 1st component of the union as `number`, and `any` is not exactly `number`\n",
            "\t * in the 1st component of the intersection, accessing `q` results in `any` and accessing `q` has the 2nd component of the union as `nil`, and `any` is not exactly `nil`\n",
            "\t * in the 2nd component of the intersection, accessing `p` results in `unknown` and accessing `p` has the 1st component of the union as `string`, and `unknown` is not exactly `string`\n",
            "\t * in the 2nd component of the intersection, accessing `p` results in `unknown` and accessing `p` has the 2nd component of the union as `nil`, and `unknown` is not exactly `nil`\n",
            "\t * in the 2nd component of the intersection, accessing `q` has the 1st component of the union as `string` and accessing `q` has the 1st component of the union as `number`, and `string` is not exactly `number`\n",
            "\t * in the 2nd component of the intersection, accessing `q` has the 1st component of the union as `string` and accessing `q` has the 2nd component of the union as `nil`, and `string` is not exactly `nil`\n",
            "\t * in the 2nd component of the intersection, accessing `q` has the 2nd component of the union as `nil` and accessing `q` has the 1st component of the union as `number`, and `nil` is not exactly `number`"
        )
    } else {
        r#"Expected this to be
	'{ p: string?, q: number? }'
but got
	'{ p: number?, q: any } & { p: unknown, q: string? }'; none of the intersection parts are compatible"#
    };

    if FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
