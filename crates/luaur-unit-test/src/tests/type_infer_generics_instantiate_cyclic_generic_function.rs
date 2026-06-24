//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1252:type_infer_generics_instantiate_cyclic_generic_function`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_generics_instantiate_cyclic_generic_function

#[cfg(test)]
#[test]
fn type_infer_generics_instantiate_cyclic_generic_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(o)
            o:method()
        end

        function g(o)
            f(o)
        end
    "#,
        ),
        None,
    );

    let g = fixture.require_type_string(&String::from("g"));
    let g_fun = unsafe { get_type_id::<FunctionType>(g).as_ref() }.expect("expected FunctionType");

    let arg = first(g_fun.arg_types(), false).expect("expected argument type");
    let arg = unsafe { follow_type_id(arg) };
    let arg_table = unsafe { get_type_id::<TableType>(arg).as_ref() }
        .unwrap_or_else(|| panic!("expected table but got {}", to_string_type_id(arg)));

    let method_prop = arg_table
        .props
        .get(&String::from("method"))
        .expect("expected method property");
    let method_ty = method_prop.read_ty.expect("expected readable method type");
    let method_ty = unsafe { follow_type_id(method_ty) };
    let method_function = unsafe { get_type_id::<FunctionType>(method_ty).as_ref() }
        .expect("expected method FunctionType");

    let method_arg = first(method_function.arg_types(), false).expect("expected method argument");
    assert_eq!(unsafe { follow_type_id(method_arg) }, unsafe {
        follow_type_id(arg)
    });
}
