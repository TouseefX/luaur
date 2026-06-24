//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_scrub_unsealed_tables() {
    use crate::functions::has_error::has_error;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::records::cannot_extend_table::CannotExtendTable;
    use luaur_analysis::records::code_too_complex::CodeTooComplex;
    use luaur_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
    use luaur_analysis::records::internal_error::InternalError;
    use luaur_common::{FFlag, FInt};

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _constraint_limit = ScopedFastInt::new(&FInt::LuauSolverConstraintLimit, 5);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        type Array<T> = {T}
        type Hello = Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<number>>>>>>>>>>
        local X = {}
        X.foo = 42
        X.bar = ""
        return X
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        local x = require(game.A)
        x.lmao = 42
        return {}
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
    assert!(
        has_error::<ConstraintSolvingIncompleteError>(&result),
        "{:?}",
        result.errors
    );
    assert!(has_error::<InternalError>(&result), "{:?}", result.errors);
    assert!(
        has_error::<CannotExtendTable>(&result),
        "{:?}",
        result.errors
    );
}
