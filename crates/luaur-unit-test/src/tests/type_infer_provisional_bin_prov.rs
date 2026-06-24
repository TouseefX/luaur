//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1214:type_infer_provisional_bin_prov`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_provisional_bin_prov

#[cfg(test)]
#[test]
fn type_infer_provisional_bin_prov() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Bin = {}

        function Bin:add(item)
            self.head = { item = item}
            return item
        end

        function Bin:destroy()
            while self.head do
                local item = self.head.item
                if type(item) == "function" then
                    item()
                elseif item.Destroy ~= nil then
                end
                self.head = self.head.next
            end
        end
    "#,
        ),
        None,
    );
}
