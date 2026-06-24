#[cfg(test)]
#[test]
fn type_infer_tables_oss_1859() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        type Cat = {
            name: string,
            age: number,
            actions: {
                otherfield: string,
                meow: () -> string,
            }
        }

        local function new(): Cat
            local self = {}
            self.name = "Taz"
            self.age = 12
            self.actions = {}
            self.actions.meow = function() return "meow" end
            -- We're missing `otherfield` here so we should complain.
            return self
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("Cat", to_string_type_id(err.wanted_type));
    assert_eq!(
        "{ actions: { meow: (...any) -> string }, age: number, name: string }",
        to_string_type_id(err.given_type)
    );
}
