//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1878:type_infer_generics_follow_bound_type_packs_in_generic_type_visitor`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item type_infer_generics_follow_bound_type_packs_in_generic_type_visitor

#[cfg(test)]
#[test]
fn type_infer_generics_follow_bound_type_packs_in_generic_type_visitor() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
function (_(_,_,nil))
(if l0 then typeof else `{_:_()}`,typeof).n0<A...,A...>(l0)
function _:_():typeof<A...>()
end
function _:_().typeof<A...>()
end
end
    "#,
        ),
        None,
    );
}
