//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:349:type_infer_exponential_blowup_from_copying_types`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_exponential_blowup_from_copying_types

#[cfg(test)]
#[test]
fn type_infer_exponential_blowup_from_copying_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        -- An example of exponential blowup in number of types
        -- The problem is that if we define function f(a) return x end
        -- then this has type <t>(t)->T where x:T
        -- *but* it copies T each time f is applied
        -- so { left = f("hi"), right = f(5) }
        -- has type { left : T_L, right : T_R }
        -- where T_L and T_R are copies of T.
        -- x0 : T0 where T0 = {}
        local x0 = {}
        -- f0 : <t>(t)->T0
        local function f0(a) return x0 end
        -- x1 : T1 where T1 = { left : T0_L, right : T0_R }
        local x1 = { left = f0("hi"), right = f0(5) }
        -- f1 : <t>(t)->T1
        local function f1(a) return x1 end
        -- x2 : T2 where T2 = { left : T1_L, right : T1_R }
        local x2 = { left = f1("hi"), right = f1(5) }
        -- f2 : <t>(t)->T2
        local function f2(a) return x2 end
        -- etc etc
        local x3 = { left = f2("hi"), right = f2(5) }
        local function f3(a) return x3 end
        local x4 = { left = f3("hi"), right = f3(5) }
        return x4
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    assert!(
        5 >= module.interface_types.types.size(),
        "interface type count was {}",
        module.interface_types.types.size()
    );
}
