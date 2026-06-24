//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:763:type_infer_functions_higher_order_function_4`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function min (Analysis/include/Luau/Unifiable.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_functions_higher_order_function_4

#[cfg(test)]
#[test]
fn type_infer_functions_higher_order_function_4() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::size_type_pack::size;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function bottomupmerge(comp, a, b, left, mid, right)
            local i, j = left, mid
            for k = left, right do
                if i < mid and (j > right or not comp(a[j], a[i])) then
                    b[k] = a[i]
                    i = i + 1
                else
                    b[k] = a[j]
                    j = j + 1
                end
            end
        end

        function mergesort<T>(arr: {T}, comp: (T, T) -> boolean)
            local work = {}
            for i = 1, #arr do
                work[i] = arr[i]
            end
            local width = 1
            while width < #arr do
                for i = 1, #arr, 2*width do
                    bottomupmerge(comp, arr, work, i, math.min(i+width, #arr), math.min(i+2*width-1, #arr))
                end
                local temp = work
                work = arr
                arr = temp
                width = width * 2
            end
            return arr
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ftv = unsafe {
        get_type_id::<FunctionType>(fixture.base.require_type_string(&String::from("mergesort")))
            .as_ref()
    }
    .expect("expected mergesort to have function type");

    let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(2, arg_vec.len());

    let arg0 = unsafe { get_type_id::<TableType>(follow_type_id(arg_vec[0])).as_ref() }
        .expect("expected first argument to be a table");
    let indexer = arg0.indexer.as_ref().expect("expected table indexer");

    let arg1 = unsafe { get_type_id::<FunctionType>(follow_type_id(arg_vec[1])).as_ref() }
        .expect("expected second argument to be a function");
    assert_eq!(2, size(arg1.arg_types(), core::ptr::null_mut()));

    let (arg1_args, _) = flatten_type_pack_id(arg1.arg_types());

    assert_eq!(
        unsafe { follow_type_id(indexer.index_result_type) },
        unsafe { follow_type_id(arg1_args[0]) }
    );
    assert_eq!(
        unsafe { follow_type_id(indexer.index_result_type) },
        unsafe { follow_type_id(arg1_args[1]) }
    );
}
