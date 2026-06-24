//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:665:type_infer_functions_infer_higher_order_function`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item type_infer_functions_infer_higher_order_function

#[cfg(test)]
#[test]
fn type_infer_functions_infer_higher_order_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function apply(f, x)
            return f(x)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let apply_type = fixture.require_type_string(&String::from("apply"));
    let ftv = unsafe { get_type_id::<FunctionType>(apply_type).as_ref() }
        .expect("expected apply to have function type");

    let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(2, arg_vec.len());

    let f_arg_type = unsafe { follow_type_id(arg_vec[0]) };
    let f_type = unsafe { get_type_id::<FunctionType>(f_arg_type).as_ref() }.unwrap_or_else(|| {
        panic!(
            "expected a function but got {}",
            to_string_type_id(arg_vec[0])
        )
    });

    let (f_args, _) = flatten_type_pack_id(f_type.arg_types());
    let x_type = unsafe { follow_type_id(arg_vec[1]) };

    assert_eq!(1, f_args.len());
    assert_eq!(x_type, unsafe { follow_type_id(f_args[0]) });
}
