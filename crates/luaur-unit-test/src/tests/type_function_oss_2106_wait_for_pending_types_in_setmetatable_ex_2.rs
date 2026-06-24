//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1984:type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_2`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_2

#[cfg(test)]
#[test]
fn type_function_oss_2106_wait_for_pending_types_in_setmetatable_ex_2() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
        ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local MyClass = {}
        local MyClassMetatable = { __index = MyClass }
        table.freeze(MyClassMetatable)

        type CommonFields<T> = { read name: T }
        type MyClass = setmetatable<CommonFields<string>, typeof(MyClassMetatable)>

        function MyClass.new(name: string): MyClass
            return setmetatable({ name = name }, MyClassMetatable)
        end

        function MyClass.hello(self: MyClass): string
            return `Hello, {self.name}!`
        end

        local instance = MyClass.new("World")
        local g = instance:hello()
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("g")))
    );
}
