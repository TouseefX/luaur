//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_ensure_scope_is_nullptr_after_shallow_copy() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
-- Roughly taken from ReactTypes.lua
type CoreBinding<T> = {}
type BindingMap = {}
export type Binding<T> = CoreBinding<T> & BindingMap

return {}
    "#,
        ),
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local Types = require(game.A)
type Binding<T> = Types.Binding<T>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
