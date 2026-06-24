//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:678:type_infer_oop_oop_invoke_with_inferred_self_type`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item type_infer_oop_oop_invoke_with_inferred_self_type

#[cfg(test)]
#[test]
fn type_infer_oop_oop_invoke_with_inferred_self_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local ItemContainer = {}
        ItemContainer.__index = ItemContainer

        function ItemContainer.new()
            local self = {}
            setmetatable(self, ItemContainer)
            return self
        end

        function ItemContainer:removeItem(itemId, itemType)
            self:getItem(itemId, itemType)
        end

        function ItemContainer:getItem(itemId, itemType): ()
        end

        local container = ItemContainer.new()

        container:removeItem(0, "magic")
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
