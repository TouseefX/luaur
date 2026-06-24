#[cfg(test)]
#[test]
fn type_infer_tables_oss_1450() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let results = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local keycodes = {
            Alt = 2,
            Space = 3,
            Tab = 4,
        }

        type Keycode = keyof<typeof(keycodes)>
        local function sendInput(keycodes: { Keycode })
            print(keycodes)
        end

        sendInput({"Alt"}) -- shouldn't error
        sendInput(
            {
                "Alt",
                "Space",
                "Ctrl", -- should error
            }
        )
    "#,
        ),
        None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    let err =
        type_error_data_ref::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
        Location::new(Position::new(17, 16), Position::new(17, 22)),
        results.errors[0].location
    );
    assert_eq!(
        "\"Alt\" | \"Space\" | \"Tab\"",
        to_string_type_id(err.wanted_type)
    );
    assert_eq!("\"Ctrl\"", to_string_type_id(err.given_type));
}
