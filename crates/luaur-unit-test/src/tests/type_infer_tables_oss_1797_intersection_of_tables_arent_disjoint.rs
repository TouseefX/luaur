//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5958:type_infer_tables_oss_1797_intersection_of_tables_arent_disjoint`
//! Source: `tests/TypeInfer.tables.test.cpp`

#[cfg(test)]
#[test]
fn type_infer_tables_oss_1797_intersection_of_tables_arent_disjoint() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        export type Foo = {
            foo: string,
        }

        export type Bar = Foo & {
            copy: (...any) -> any
        }

        local function _test(nd: { bar: Bar? })
            local bar = nd.bar
            if not bar then
                return
            end
            print(bar)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Foo & { copy: (...any) -> any }",
        to_string_type_id(
            fixture
                .base
                .require_type_at_position_position(Position::new(16, 20))
        )
    );
}
