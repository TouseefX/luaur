//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2962:type_infer_tables_table_unifies_into_map`
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
//!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_table_unifies_into_map

#[cfg(test)]
#[test]
fn type_infer_tables_table_unifies_into_map() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Instance: any
        local UDim2: any

        function Create(instanceType)
            return function(data)
                local obj = Instance.new(instanceType)
                for k, v in pairs(data) do
                    if type(k) == 'number' then
                        --v.Parent = obj
                    else
                        obj[k] = v
                    end
                end
                return obj
            end
        end

        local topbarShadow = Create'ImageLabel'{
            Name = "TopBarShadow";
            Size = UDim2.new(1, 0, 0, 3);
            Position = UDim2.new(0, 0, 1, 0);
            Image = "rbxasset://textures/ui/TopBar/dropshadow.png";
            BackgroundTransparency = 1;
            Active = false;
            Visible = false;
        };

    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
