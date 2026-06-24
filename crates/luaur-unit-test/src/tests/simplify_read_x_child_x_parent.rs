//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:657:simplify_read_x_child_x_parent`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - calls -> function createSomeExternTypes (tests/Fixture.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_read_x_child_x_parent

#[cfg(test)]
#[test]
fn simplify_read_x_child_x_parent() {
    use crate::functions::create_some_extern_types::create_some_extern_types;
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::sync::Arc;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::scope::Scope;

    let mut fixture = SimplifyFixture::default();

    let (parent_ty, child_ty) = {
        let frontend = fixture.base.get_frontend();
        create_some_extern_types(frontend);

        let module_scope = frontend.globals.global_scope();
        let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

        unsafe {
            (
                (*module_scope_ptr)
                    .exported_type_bindings
                    .get("Parent")
                    .expect("Parent exported type binding")
                    .r#type(),
                (*module_scope_ptr)
                    .exported_type_bindings
                    .get("Child")
                    .expect("Child exported type binding")
                    .r#type(),
            )
        }
    };

    let left_ty = fixture.mk_table_props(&[("x", Property::readonly(child_ty))]);
    let right_ty = fixture.mk_table(&[("x", parent_ty)]);

    let actual = fixture.intersect(left_ty, right_ty);
    assert_eq!(
        "{ read x: Child } & { x: Parent }",
        to_string_type_id_to_string_options(actual, &mut fixture.opts)
    );
}
