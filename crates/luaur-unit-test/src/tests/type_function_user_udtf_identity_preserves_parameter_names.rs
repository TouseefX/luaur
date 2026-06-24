//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2937:type_function_user_udtf_identity_preserves_parameter_names`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item type_function_user_udtf_identity_preserves_parameter_names

#[cfg(test)]
#[test]
fn type_function_user_udtf_identity_preserves_parameter_names() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _serialize_arg_names = ScopedFastFlag::new(&FFlag::LuauTypeFunctionSerializeArgNames, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type function identity(t)
    return t
end

type baz = string
type foo = (foo: number, bar: baz) -> baz
type bar = identity<foo>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let bar_ty = fixture.base.require_type_alias(&String::from("bar"));
    let ftv =
        unsafe { get_type_id::<FunctionType>(bar_ty).as_ref() }.expect("expected FunctionType");

    assert_eq!(2, ftv.arg_names().len());
    let first = ftv.arg_names()[0]
        .as_ref()
        .expect("expected first argument name");
    assert_eq!("foo", first.name);
    let second = ftv.arg_names()[1]
        .as_ref()
        .expect("expected second argument name");
    assert_eq!("bar", second.name);
}
