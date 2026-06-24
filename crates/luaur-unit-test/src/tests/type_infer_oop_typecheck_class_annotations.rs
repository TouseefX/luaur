//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1177:type_infer_oop_typecheck_class_annotations`
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
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_oop_typecheck_class_annotations

#[cfg(test)]
#[test]
fn type_infer_oop_typecheck_class_annotations() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_analysis::records::type_pack_mismatch::TypePackMismatch;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        class Point
            public x: number
            public y: number
            public name: string
            function magnitude(self): string
                -- self.name is not a number
                self.name = self.x

                -- This function is declared to return string.
                return math.sqrt(self.x * self.x + self.y * self.y)
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(result
        .errors
        .iter()
        .any(|err| type_error_data_ref::<TypeMismatch>(err).is_some()));
    assert!(result
        .errors
        .iter()
        .any(|err| type_error_data_ref::<TypePackMismatch>(err).is_some()));
}
