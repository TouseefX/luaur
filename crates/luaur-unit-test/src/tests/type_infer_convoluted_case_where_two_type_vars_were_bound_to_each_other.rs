//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1474:type_infer_convoluted_case_where_two_type_vars_were_bound_to_each_other`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
//!   - type_ref -> record Config (Config/include/Luau/Config.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> method WeirdIter::good (Analysis/src/Unifier.cpp)
//!   - translates_to -> rust_item type_infer_convoluted_case_where_two_type_vars_were_bound_to_each_other

#[cfg(test)]
#[test]
fn type_infer_convoluted_case_where_two_type_vars_were_bound_to_each_other() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type React_Ref<ElementType> = { current: ElementType } | ((ElementType) -> ())

        type React_AbstractComponent<Config, Instance> = {
            render: ((ref: React_Ref<Instance>) -> nil)
        }

        local createElement : <P, T>(React_AbstractComponent<P, T>) -> ()

        function ScrollView:render()
            local one = table.unpack(
                if true then a else b
            )

            createElement(one)
            createElement(one)
        end
    "#,
        ),
        None,
    );
}
