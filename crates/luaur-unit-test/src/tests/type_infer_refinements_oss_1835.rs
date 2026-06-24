//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2749:type_infer_refinements_oss_1835`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_refinements_oss_1835

#[cfg(test)]
#[test]
fn type_infer_refinements_oss_1835() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::records::optional_value_access::OptionalValueAccess;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local t: {name: string}? = nil

        function f()
            local name = if t then t.name else "name"
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local t: {name: string}? = nil

        function f()
            if t then end
            local name = if t then t.name else "name"
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t: {name: string}? = nil
        if t then end
        print(t.name)
        local name = if t then t.name else "name"
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<OptionalValueAccess>(&result.errors[0])
        .expect("expected OptionalValueAccess");
}
