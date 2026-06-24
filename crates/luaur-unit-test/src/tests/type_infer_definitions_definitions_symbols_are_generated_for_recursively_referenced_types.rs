//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:362:type_infer_definitions_definitions_symbols_are_generated_for_recursively_referenced_types`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_definitions_definitions_symbols_are_generated_for_recursively_referenced_types

#[cfg(test)]
#[test]
fn type_infer_definitions_definitions_symbols_are_generated_for_recursively_referenced_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare class MyClass
            function myMethod(self)
        end

        declare function myFunc(): MyClass
    "#,
        ),
        false,
    );

    let frontend = fixture.get_frontend();
    let my_class_ty = frontend
        .globals
        .global_scope()
        .lookup_type(&String::from("MyClass"))
        .expect("MyClass type binding");
    assert_eq!(Some(String::from("@test/globaltype/MyClass")), unsafe {
        (*my_class_ty.r#type()).documentation_symbol.clone()
    });

    let class = unsafe { get_mutable_type_id::<ExternType>(my_class_ty.r#type()) };
    assert!(!class.is_null(), "expected MyClass extern type");
    let method = unsafe { (*class).props.get("myMethod").expect("myMethod") };
    assert_eq!(
        Some(String::from("@test/globaltype/MyClass.myMethod")),
        method.documentation_symbol.clone()
    );

    let method_ty = method.read_ty.expect("myMethod read type");
    let function = unsafe { get_mutable_type_id::<FunctionType>(method_ty) };
    assert!(!function.is_null(), "expected myMethod function type");
    let definition = unsafe {
        (*function)
            .definition()
            .expect("myMethod function definition")
    };

    assert_eq!(
        Some(&String::from("@test")),
        definition.definitionModuleName()
    );
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 12,
            },
            end: Position {
                line: 2,
                column: 35,
            },
        },
        definition.definitionLocation()
    );
    assert_eq!(None, definition.varargLocation());
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 21,
            },
            end: Position {
                line: 2,
                column: 29,
            },
        },
        definition.originalNameLocation()
    );
}
