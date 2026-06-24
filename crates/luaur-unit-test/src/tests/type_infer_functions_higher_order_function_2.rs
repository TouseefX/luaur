//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:693:type_infer_functions_higher_order_function_2`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item type_infer_functions_higher_order_function_2

#[cfg(test)]
#[test]
fn type_infer_functions_higher_order_function_2() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function bottomupmerge(comp, a, b, left, mid, right)
            local i, j = left, mid
            for k = left, right do
                if i < mid and (j > right or not comp(a[j], a[i])) then
                    b[k] = a[i]
                    i = i + 1
                else
                    b[k] = a[j]
                    j = j + 1
                end
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ftv = unsafe {
        get_type_id::<FunctionType>(fixture.require_type_string(&String::from("bottomupmerge")))
            .as_ref()
    }
    .expect("expected bottomupmerge to have function type");

    let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(6, arg_vec.len());

    let f_type = unsafe { get_type_id::<FunctionType>(follow_type_id(arg_vec[0])).as_ref() };
    assert!(f_type.is_some(), "expected first argument to be a function");
}
