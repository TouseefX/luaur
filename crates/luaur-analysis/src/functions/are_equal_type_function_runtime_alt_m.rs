use crate::functions::are_equal_type_function_runtime::are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type;
use crate::functions::are_equal_type_function_runtime_alt_g::are_equal_are_equal_state_type_function_union_type_type_function_union_type;
use crate::functions::are_equal_type_function_runtime_alt_h::are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type;
use crate::functions::are_equal_type_function_runtime_alt_i::are_equal_are_equal_state_type_function_negation_type_type_function_negation_type;
use crate::functions::are_equal_type_function_runtime_alt_j::are_equal_are_equal_state_type_function_table_type_type_function_table_type;
use crate::functions::are_equal_type_function_runtime_alt_k::are_equal_are_equal_state_type_function_function_type_type_function_function_type;
use crate::functions::are_equal_type_function_runtime_alt_l::are_equal_are_equal_state_type_function_extern_type_type_function_extern_type;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::records::are_equal_state::AreEqualState;
use crate::records::recursion_limiter::RecursionLimiter;
use crate::records::type_function_any_type::TypeFunctionAnyType;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;
use crate::records::type_function_negation_type::TypeFunctionNegationType;
use crate::records::type_function_never_type::TypeFunctionNeverType;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::records::type_function_type::TypeFunctionType;
use crate::records::type_function_union_type::TypeFunctionUnionType;
use crate::records::type_function_unknown_type::TypeFunctionUnknownType;
use crate::FFlag::LuauTypeFunctionRobustness;

#[allow(non_snake_case)]
pub fn are_equal_are_equal_state_type_function_type_type_function_type(
    seen: &mut AreEqualState,
    lhs: &TypeFunctionType,
    rhs: &TypeFunctionType,
) -> bool {
    let mut _ra: Option<RecursionLimiter> = None;
    if LuauTypeFunctionRobustness.get() {
        let mut rl = RecursionLimiter {
            base: unsafe { core::mem::zeroed() },
            native_stack_guard: unsafe { core::mem::zeroed() },
        };
        rl.recursion_limiter_recursion_limiter(
            "areEqual",
            &mut seen.recursion_count as *mut core::ffi::c_int,
            100,
        );
        _ra = Some(rl);
    }

    if lhs.type_variant.index() != rhs.type_variant.index() {
        return false;
    }

    {
        let lp = unsafe {
            get_type_function_type_id::<TypeFunctionPrimitiveType>(lhs as *const TypeFunctionType)
        };
        let rp = unsafe {
            get_type_function_type_id::<TypeFunctionPrimitiveType>(rhs as *const TypeFunctionType)
        };
        if !lp.is_null() && !rp.is_null() {
            return unsafe { (*lp).r#type == (*rp).r#type };
        }
    }

    {
        let _ = unsafe {
            get_type_function_type_id::<TypeFunctionAnyType>(lhs as *const TypeFunctionType)
        };
        let _ = unsafe {
            get_type_function_type_id::<TypeFunctionAnyType>(rhs as *const TypeFunctionType)
        };
        if !unsafe {
            get_type_function_type_id::<TypeFunctionAnyType>(lhs as *const TypeFunctionType)
        }
        .is_null()
            && !unsafe {
                get_type_function_type_id::<TypeFunctionAnyType>(rhs as *const TypeFunctionType)
            }
            .is_null()
        {
            return true;
        }
    }

    {
        let _ = unsafe {
            get_type_function_type_id::<TypeFunctionUnknownType>(lhs as *const TypeFunctionType)
        };
        let _ = unsafe {
            get_type_function_type_id::<TypeFunctionUnknownType>(rhs as *const TypeFunctionType)
        };
        if !unsafe {
            get_type_function_type_id::<TypeFunctionUnknownType>(lhs as *const TypeFunctionType)
        }
        .is_null()
            && !unsafe {
                get_type_function_type_id::<TypeFunctionUnknownType>(rhs as *const TypeFunctionType)
            }
            .is_null()
        {
            return true;
        }
    }

    {
        let _ = unsafe {
            get_type_function_type_id::<TypeFunctionNeverType>(lhs as *const TypeFunctionType)
        };
        let _ = unsafe {
            get_type_function_type_id::<TypeFunctionNeverType>(rhs as *const TypeFunctionType)
        };
        if !unsafe {
            get_type_function_type_id::<TypeFunctionNeverType>(lhs as *const TypeFunctionType)
        }
        .is_null()
            && !unsafe {
                get_type_function_type_id::<TypeFunctionNeverType>(rhs as *const TypeFunctionType)
            }
            .is_null()
        {
            return true;
        }
    }

    {
        let lf = unsafe {
            get_type_function_type_id::<TypeFunctionSingletonType>(lhs as *const TypeFunctionType)
        };
        let rf = unsafe {
            get_type_function_type_id::<TypeFunctionSingletonType>(rhs as *const TypeFunctionType)
        };
        if !lf.is_null() && !rf.is_null() {
            return are_equal_are_equal_state_type_function_singleton_type_type_function_singleton_type(
                seen,
                unsafe { &*lf },
                unsafe { &*rf },
            );
        }
    }

    {
        let lf = unsafe {
            get_type_function_type_id::<TypeFunctionUnionType>(lhs as *const TypeFunctionType)
        };
        let rf = unsafe {
            get_type_function_type_id::<TypeFunctionUnionType>(rhs as *const TypeFunctionType)
        };
        if !lf.is_null() && !rf.is_null() {
            return are_equal_are_equal_state_type_function_union_type_type_function_union_type(
                seen,
                unsafe { &*lf },
                unsafe { &*rf },
            );
        }
    }

    {
        let lf = unsafe {
            get_type_function_type_id::<TypeFunctionIntersectionType>(
                lhs as *const TypeFunctionType,
            )
        };
        let rf = unsafe {
            get_type_function_type_id::<TypeFunctionIntersectionType>(
                rhs as *const TypeFunctionType,
            )
        };
        if !lf.is_null() && !rf.is_null() {
            return are_equal_are_equal_state_type_function_intersection_type_type_function_intersection_type(
                seen,
                unsafe { &*lf },
                unsafe { &*rf },
            );
        }
    }

    {
        let lf = unsafe {
            get_type_function_type_id::<TypeFunctionNegationType>(lhs as *const TypeFunctionType)
        };
        let rf = unsafe {
            get_type_function_type_id::<TypeFunctionNegationType>(rhs as *const TypeFunctionType)
        };
        if !lf.is_null() && !rf.is_null() {
            return are_equal_are_equal_state_type_function_negation_type_type_function_negation_type(
                seen,
                unsafe { &*lf },
                unsafe { &*rf },
            );
        }
    }

    {
        let lt = unsafe {
            get_type_function_type_id::<TypeFunctionTableType>(lhs as *const TypeFunctionType)
        };
        let rt = unsafe {
            get_type_function_type_id::<TypeFunctionTableType>(rhs as *const TypeFunctionType)
        };
        if !lt.is_null() && !rt.is_null() {
            return are_equal_are_equal_state_type_function_table_type_type_function_table_type(
                seen,
                unsafe { &*lt },
                unsafe { &*rt },
            );
        }
    }

    {
        let lf = unsafe {
            get_type_function_type_id::<TypeFunctionFunctionType>(lhs as *const TypeFunctionType)
        };
        let rf = unsafe {
            get_type_function_type_id::<TypeFunctionFunctionType>(rhs as *const TypeFunctionType)
        };
        if !lf.is_null() && !rf.is_null() {
            return are_equal_are_equal_state_type_function_function_type_type_function_function_type(
                seen,
                unsafe { &*lf },
                unsafe { &*rf },
            );
        }
    }

    {
        let lf = unsafe {
            get_type_function_type_id::<TypeFunctionExternType>(lhs as *const TypeFunctionType)
        };
        let rf = unsafe {
            get_type_function_type_id::<TypeFunctionExternType>(rhs as *const TypeFunctionType)
        };
        if !lf.is_null() && !rf.is_null() {
            return are_equal_are_equal_state_type_function_extern_type_type_function_extern_type(
                seen,
                unsafe { &*lf },
                unsafe { &*rf },
            );
        }
    }

    {
        let lg = unsafe {
            get_type_function_type_id::<TypeFunctionGenericType>(lhs as *const TypeFunctionType)
        };
        let rg = unsafe {
            get_type_function_type_id::<TypeFunctionGenericType>(rhs as *const TypeFunctionType)
        };
        if !lg.is_null() && !rg.is_null() {
            return unsafe {
                (*lg).is_named == (*rg).is_named
                    && (*lg).is_pack == (*rg).is_pack
                    && (*lg).name == (*rg).name
            };
        }
    }

    false
}
