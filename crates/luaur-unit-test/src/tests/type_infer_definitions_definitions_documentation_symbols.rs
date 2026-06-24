//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:318:type_infer_definitions_definitions_documentation_symbols`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> function linearSearchForBinding (tests/Fixture.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_definitions_definitions_documentation_symbols

#[cfg(test)]
#[test]
fn type_infer_definitions_definitions_documentation_symbols() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::try_get_global_binding::try_get_global_binding;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare x: string

        export type Foo = string | number

        declare class Bar
            prop: string
        end

        declare y: {
            x: number,
        }
    "#,
        ),
        false,
    );

    let frontend = fixture.get_frontend();

    let x_binding = try_get_global_binding(&mut frontend.globals, "x").expect("x binding");
    assert_eq!(
        Some(String::from("@test/global/x")),
        x_binding.documentation_symbol
    );

    let foo_ty = frontend
        .globals
        .global_scope()
        .lookup_type(&String::from("Foo"))
        .expect("Foo type binding");
    assert_eq!(Some(String::from("@test/globaltype/Foo")), unsafe {
        (*foo_ty.r#type()).documentation_symbol.clone()
    });

    let bar_ty = frontend
        .globals
        .global_scope()
        .lookup_type(&String::from("Bar"))
        .expect("Bar type binding");
    assert_eq!(Some(String::from("@test/globaltype/Bar")), unsafe {
        (*bar_ty.r#type()).documentation_symbol.clone()
    });

    let bar_class = unsafe { get_mutable_type_id::<ExternType>(bar_ty.r#type()) };
    assert!(!bar_class.is_null(), "expected Bar extern type");
    let bar_prop = unsafe { (*bar_class).props.get("prop").expect("Bar.prop") };
    assert_eq!(
        Some(String::from("@test/globaltype/Bar.prop")),
        bar_prop.documentation_symbol.clone()
    );

    let y_binding = try_get_global_binding(&mut frontend.globals, "y").expect("y binding");
    assert_eq!(
        Some(String::from("@test/global/y")),
        y_binding.documentation_symbol
    );

    let y_table = unsafe { get_mutable_type_id::<TableType>(y_binding.type_id) };
    assert!(!y_table.is_null(), "expected y table type");
    let y_prop = unsafe { (*y_table).props.get("x").expect("y.x") };
    assert_eq!(
        Some(String::from("@test/global/y.x")),
        y_prop.documentation_symbol.clone()
    );
}
