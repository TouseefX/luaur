//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:317:type_infer_generics_infer_generic_function`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> method PathBuilder::rets (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item type_infer_generics_infer_generic_function

#[cfg(test)]
#[test]
fn type_infer_generics_infer_generic_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id(x)
            return x
        end
        local x: string = id("hi")
        local y: number = id(37)
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let id_type = fixture.require_type_string(&String::from("id"));
    let id_fun =
        unsafe { get_type_id::<FunctionType>(id_type).as_ref() }.expect("expected FunctionType");
    let (args, _) = flatten_type_pack_id(id_fun.arg_types());
    let (rets, _) = flatten_type_pack_id(id_fun.ret_types());

    assert_eq!(1, id_fun.generics().len());
    assert_eq!(0, id_fun.generic_packs().len());
    assert_eq!(unsafe { follow_type_id(args[0]) }, unsafe {
        follow_type_id(id_fun.generics()[0])
    });
    assert_eq!(unsafe { follow_type_id(rets[0]) }, unsafe {
        follow_type_id(id_fun.generics()[0])
    });
}
