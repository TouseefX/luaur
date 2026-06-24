//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:41:type_infer_provisional_typeguard_inference_incomplete`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method Fixture::decorateWithTypes (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_provisional_typeguard_inference_incomplete

#[cfg(test)]
#[test]
fn type_infer_provisional_typeguard_inference_incomplete() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let code = String::from(
        r#"
        function f(a)
            if type(a) == "boolean" then
                local a1 = a
            elseif a.fn() then
                local a2 = a
            end
        end
    "#,
    );

    let expected = r#"
        function f(a:{fn:()->(a,b...)}): ()
            if type(a) == 'boolean' then
                local a1:boolean=a
            elseif a.fn() then
                local a2:{fn:()->(a,b...)}=a
            end
        end
    "#;

    let expected_with_new_solver = r#"
        function f(a:{fn:()->(unknown,...unknown)}): ()
            if type(a) == 'boolean' then
                local a1:{fn:()->(unknown,...unknown)}&boolean=a
            elseif a.fn() then
                local a2:{fn:()->(unknown,...unknown)}&(userdata|function|nil|number|integer|string|thread|buffer|table)=a
            end
        end
    "#;

    let expected_with_new_solver_nointeger = r#"
        function f(a:{fn:()->(unknown,...unknown)}): ()
            if type(a) == 'boolean' then
                local a1:{fn:()->(unknown,...unknown)}&boolean=a
            elseif a.fn() then
                local a2:{fn:()->(unknown,...unknown)}&(userdata|function|nil|number|string|thread|buffer|table)=a
            end
        end
    "#;

    let mut fixture = Fixture::fixture_bool(false);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        if FFlag::LuauIntegerType2.get() {
            expected_with_new_solver
        } else {
            expected_with_new_solver_nointeger
        }
    } else {
        expected
    };

    assert_eq!(expected, fixture.decorate_with_types(&code));
}
