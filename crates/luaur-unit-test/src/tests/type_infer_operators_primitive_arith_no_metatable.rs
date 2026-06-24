//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:115:type_infer_operators_primitive_arith_no_metatable`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> macro tonumber (VM/src/lvm.h)
//!   - calls -> macro tostring (VM/src/lvm.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item type_infer_operators_primitive_arith_no_metatable

#[cfg(test)]
#[test]
fn type_infer_operators_primitive_arith_no_metatable() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function add(a: number, b: string)
            return a + (tonumber(b) :: number), tostring(a) .. b
        end
        local n, s = add(2,"3")
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let function_type = fixture.base.require_type_string(&String::from("add"));
    let function_type =
        unsafe { get_type_id::<FunctionType>(follow_type_id(function_type)).as_ref() }
            .expect("expected FunctionType");
    let ret_type = first(function_type.ret_types(), false).expect("expected first return type");
    assert_eq!("number", to_string_type_id(ret_type));
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("n")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("s")))
    );
}
