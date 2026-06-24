//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1511:type_infer_handle_self_referential_has_prop_constraints`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item type_infer_handle_self_referential_has_prop_constraints

#[cfg(test)]
#[test]
fn type_infer_handle_self_referential_has_prop_constraints() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function calculateTopBarHeight(props)
        end
        local function isTopPage(props)
            local topMostOpaquePage
            if props.avatarRoute then
                topMostOpaquePage = props.avatarRoute.opaque.name
            else
                topMostOpaquePage = props.opaquePage
            end
        end

        function TopBarContainer:updateTopBarHeight(prevProps, prevState)
            calculateTopBarHeight(self.props)
            isTopPage(self.props)
            local topMostOpaquePage
            if self.props.avatarRoute then
                topMostOpaquePage = self.props.avatarRoute.opaque.name
                --                  ^--------------------------------^
            else
                topMostOpaquePage = self.props.opaquePage
            end
        end
    "#,
        ),
        None,
    );
}
