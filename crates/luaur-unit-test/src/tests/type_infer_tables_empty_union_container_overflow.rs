//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5480:type_infer_tables_empty_union_container_overflow`
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
//!   - translates_to -> rust_item type_infer_tables_empty_union_container_overflow

#[cfg(test)]
#[test]
fn type_infer_tables_empty_union_container_overflow() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local CellRenderer = {}
        function CellRenderer:init(props)
            self._separators = {
                unhighlight = function()
                    local cellKey, prevCellKey = self.props.cellKey, self.props.prevCellKey
                    self.props.onUpdateSeparators({ cellKey, prevCellKey })
                end,
                updateProps = function (select, newProps)
                    local cellKey, prevCellKey = self.props.cellKey, self.props.prevCellKey
                    self.props.onUpdateSeparators({ if select == 'leading' then prevCellKey else cellKey })
                end
            }
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
