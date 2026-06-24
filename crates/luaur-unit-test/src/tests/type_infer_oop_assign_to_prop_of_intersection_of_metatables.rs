//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:790:type_infer_oop_assign_to_prop_of_intersection_of_metatables`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_oop_assign_to_prop_of_intersection_of_metatables

#[cfg(test)]
#[test]
fn type_infer_oop_assign_to_prop_of_intersection_of_metatables() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _fix_prop_reads = ScopedFastFlag::new(&FFlag::LuauFixPropReadsOnMetatableTypes, true);
    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        local Base = {}
        Base.__index = Base

        type BaseStructure = { BaseString: string }

        export type Base = setmetatable<BaseStructure, typeof(Base)>

        function Base.new() : Base
            return nil :: any
        end

        local Sub = {}
        Sub.__index = Sub

        type SubStructure = { SubString: string }

        type Sub = setmetatable<SubStructure, typeof(Sub)> & Base

        function Sub.new() : Sub
            local self: Sub = setmetatable(Base.new(), Sub) :: any

            self.SubString = 5 -- Line 24
            self.BaseString = 5 -- Line 25

            return self
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!(24, result.errors[0].location.begin.line);
    unsafe { get_type_error::<TypeMismatch>(&result.errors[1]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!(25, result.errors[1].location.begin.line);
}
