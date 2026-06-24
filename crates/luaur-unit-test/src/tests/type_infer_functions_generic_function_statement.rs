//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3194:type_infer_functions_generic_function_statement`
//! Source: `tests/TypeInfer.functions.test.cpp`

#[cfg(test)]
#[test]
fn type_infer_functions_generic_function_statement() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Object = {
            foobar: <T>(number, string, T) -> T
        }

        local Obj = {} :: Object
        function Obj.foobar(bing, quxx, dunno)
            local _ = bing
            local _ = quxx
            return dunno
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 7,
            column: 24
        }))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 8,
            column: 24
        }))
    );
    assert_eq!(
        "a",
        to_string_type_id(fixture.base.require_type_at_position_position(Position {
            line: 9,
            column: 21
        }))
    );
}
