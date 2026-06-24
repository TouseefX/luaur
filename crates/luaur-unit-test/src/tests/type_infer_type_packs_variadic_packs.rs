//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:192:type_infer_type_packs_variadic_packs`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_type_packs_variadic_packs

#[cfg(test)]
#[test]
fn type_infer_type_packs_variadic_packs() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_analysis::records::variadic_type_pack::VariadicTypePack;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();

    let number_type = fixture.get_builtins().numberType;
    let string_type = fixture.get_builtins().stringType;

    let (foo_type, bar_type) = {
        let frontend = fixture.get_frontend();
        let arena = frontend.globals.global_types_mut();

        unfreeze(arena);

        let list_of_numbers = arena.add_type_pack_t(VariadicTypePack::new(number_type));
        let list_of_strings = arena.add_type_pack_t(VariadicTypePack::new(string_type));

        let foo_rets = arena.add_type_pack_initializer_list_type_id(&[number_type]);
        let foo_type = arena.add_type(FunctionType::function_type_new(
            list_of_numbers,
            foo_rets,
            None,
            false,
        ));

        let bar_args = arena.add_type_pack_vector_type_id_optional_type_pack_id(
            alloc::vec![number_type],
            Some(list_of_strings),
        );
        let bar_rets = arena.add_type_pack_initializer_list_type_id(&[number_type]);
        let bar_type = arena.add_type(FunctionType::function_type_new(
            bar_args, bar_rets, None, false,
        ));

        (foo_type, bar_type)
    };

    {
        let frontend = fixture.get_frontend();
        add_global_binding_builtin_definitions(&mut frontend.globals, "foo", foo_type, "@test");
        add_global_binding_builtin_definitions(&mut frontend.globals, "bar", bar_type, "@test");
        freeze(frontend.globals.global_types_mut());
    }

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        foo(1, 2, 3, "foo")
        bar(1, "foo", "bar", 3)
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        Location {
            begin: Position {
                line: 3,
                column: 21
            },
            end: Position {
                line: 3,
                column: 26
            },
        },
        result.errors[0].location
    );
    assert_eq!(
        Location {
            begin: Position {
                line: 4,
                column: 29
            },
            end: Position {
                line: 4,
                column: 30
            },
        },
        result.errors[1].location
    );

    let first = type_error_data_ref::<TypeMismatch>(&result.errors[0])
        .expect("expected first error to be TypeMismatch");
    assert_eq!(number_type, unsafe { follow_type_id(first.wanted_type) });
    assert_eq!(string_type, unsafe { follow_type_id(first.given_type) });

    let second = type_error_data_ref::<TypeMismatch>(&result.errors[1])
        .expect("expected second error to be TypeMismatch");
    assert_eq!(string_type, unsafe { follow_type_id(second.wanted_type) });
    assert_eq!(number_type, unsafe { follow_type_id(second.given_type) });
}
