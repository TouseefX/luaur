#[cfg(test)]
#[test]
fn type_infer_functions_tf_suggest_arg_type_2() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::records::not_a_table::NotATable;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function escape_fslash(pre)
            return (#pre % 2 == 0 and '\\' or '') .. pre .. '.'
        end
    "#,
        ),
        None,
    );

    assert!(
        result
            .errors
            .iter()
            .any(|error| type_error_data_ref::<NotATable>(error).is_some()),
        "expected NotATable, got {:?}",
        result.errors
    );
}
