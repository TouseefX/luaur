#[cfg(test)]
#[test]
fn type_infer_functions_bidirectional_checking_of_callback_property() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_analysis::records::unknown_property::UnknownProperty;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function print(x: number) end

        type Point = {x: number, y: number}
        local T : {callback: ((Point) -> ())?} = {}

        T.callback = function(p) -- No error here
            print(p.z)           -- error here.  Point has no property z
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let tm =
            type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");

        assert_eq!("((Point) -> ())?", to_string_type_id(tm.wanted_type));
        assert_eq!(
            "({ read z: number }) -> ()",
            to_string_type_id(tm.given_type)
        );
        assert_eq!(6, result.errors[0].location.begin.line);
        assert_eq!(8, result.errors[0].location.end.line);
    } else {
        type_error_data_ref::<UnknownProperty>(&result.errors[0])
            .expect("expected UnknownProperty");
        assert_eq!(7, result.errors[0].location.begin.line);
        assert_eq!(7, result.errors[0].location.end.line);
    }
}
