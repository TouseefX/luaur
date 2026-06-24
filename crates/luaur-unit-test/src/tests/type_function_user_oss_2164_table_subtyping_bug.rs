//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:2879:type_function_user_oss_2164_table_subtyping_bug`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - translates_to -> rust_item type_function_user_oss_2164_table_subtyping_bug

#[cfg(test)]
#[test]
fn type_function_user_oss_2164_table_subtyping_bug() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _fix = ScopedFastFlag::new(&FFlag::LuauSubtypingMissingPropertiesAsNil, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type function tblpartial(tbl: type)
            assert(tbl:is("table"), "tblpartial can only be applied to tables")
            local new = types.newtable()

            for k, v in tbl:properties() do
                local read = assert(v.read, "properties cannot be write-only")
                new:setreadproperty(k, types.optional(read))
            end

            return new
        end

        local function tblmerge<T>(base: T, override: tblpartial<T>): T error("unimplemented") end
        tblmerge({ a = 1 }, {}) -- Type '{  }' could not be converted into '{ read a: number? }'
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
