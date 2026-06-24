//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:875:type_infer_oop_class_decl`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_oop_class_decl

#[cfg(test)]
#[test]
fn type_infer_oop_class_decl() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_common::FFlag;

    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        class Point
            public x: number
            public y: number
        end

        local p = Point { x = 2, y = 3 }

        local x = p.x
        local y = p.y
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = fixture.require_type_alias(&String::from("Point"));
    assert_eq!("Point", to_string_type_id(t));

    let point = unsafe { get_type_id::<ExternType>(t) };
    assert!(!point.is_null(), "expected Point alias to have ExternType");

    assert_eq!(
        "Point",
        to_string_type_id(fixture.require_type_string(&String::from("p")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("x")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("y")))
    );
}
