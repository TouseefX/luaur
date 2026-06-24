//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3069:type_infer_tables_dont_quantify_table_that_belongs_to_outer_scope`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Counter (tests/Parser.test.cpp)
//!   - calls -> method TypeLevel::incr (Analysis/include/Luau/Unifiable.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tables_dont_quantify_table_that_belongs_to_outer_scope

#[cfg(test)]
#[test]
fn type_infer_tables_dont_quantify_table_that_belongs_to_outer_scope() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::metatable_type::MetatableType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Counter = {}
        Counter.__index = Counter

        function Counter.new()
            local self = setmetatable({count=0}, Counter)
            return self
        end

        function Counter:incr()
            self.count = 1
            return self.count
        end

        local self = Counter.new()
        print(self:incr())
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let counter_ty = fixture.base.require_type_string(&String::from("Counter"));
    let counter_type =
        unsafe { get_type_id::<TableType>(counter_ty).as_ref() }.unwrap_or_else(|| {
            panic!(
                "expected Counter TableType, got {}",
                to_string_type_id(counter_ty)
            )
        });

    let new_prop = counter_type.props.get("new").expect("expected Counter.new");
    let new_prop_read_ty = new_prop.read_ty.expect("expected Counter.new read type");
    let new_type =
        unsafe { get_type_id::<FunctionType>(follow_type_id(new_prop_read_ty)).as_ref() }
            .expect("expected Counter.new FunctionType");

    let new_ret_type = first(new_type.ret_types(), false).expect("expected Counter.new return");
    let new_ret = unsafe { get_type_id::<MetatableType>(follow_type_id(new_ret_type)).as_ref() }
        .expect("expected Counter.new MetatableType return");

    let new_ret_meta =
        unsafe { get_type_id::<TableType>(follow_type_id(new_ret.metatable())).as_ref() }
            .expect("expected Counter.new metatable TableType");

    assert!(new_ret_meta.props.contains_key("incr"));
    assert_eq!(unsafe { follow_type_id(new_ret.metatable()) }, unsafe {
        follow_type_id(counter_ty)
    });
}
