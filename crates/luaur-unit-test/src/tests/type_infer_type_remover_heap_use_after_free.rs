//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2378:type_infer_type_remover_heap_use_after_free`
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
//!   - translates_to -> rust_item type_infer_type_remover_heap_use_after_free

#[cfg(test)]
#[test]
fn type_infer_type_remover_heap_use_after_free() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        _ = if l0.n0.n0 then {n4(...,setmetatable(setmetatable(_),_)),_ == _,} elseif _.ceil._ then _ elseif _ then not _
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        do
        _ = if _[_] then {[_(``)]="y",} elseif _ then _ elseif _[_] then "" elseif _ then _ elseif _[_] then {} elseif _[_] then false else ""
        end
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local l249 = require(module0)
        _,_ = {[`{_}`]=_,[_._G._]=(_)(),[_["" + _]._G]={_=_,_=_,[_._G[_]._]=_G,},},_,(_)()
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
