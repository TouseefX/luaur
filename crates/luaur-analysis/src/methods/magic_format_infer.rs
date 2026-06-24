use crate::enums::value::Value;
use crate::functions::as_mutable_type_pack_alt_d::as_mutable_type_pack;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::parse_format_string::parse_format_string;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::functions::unwrap_group::unwrap_group;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::count_mismatch::CountMismatch;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::magic_function_call_context::MagicFunctionCallContext;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::type_error::TypeError;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

pub fn magic_format_infer(context: &MagicFunctionCallContext) -> bool {
    let solver = unsafe { context.solver.as_ref() };
    let arena = unsafe { &mut *solver.arena };

    let iter = unsafe { crate::functions::begin_type_pack::begin(context.arguments) };
    let end_iter = unsafe { crate::functions::end_type_pack::end(context.arguments) };

    // we'll suppress any errors for `string.format` if the format string is error suppressing.
    if iter.operator_eq(&end_iter)
        || should_suppress_errors(solver.normalizer, unsafe {
            follow_type_id(*iter.operator_deref())
        }) == ErrorSuppression::from_value(Value::Suppress)
    {
        let result_pack = arena
            .add_type_pack_initializer_list_type_id(
                &[unsafe { &*solver.builtin_types }.stringType],
            );
        let result_mut = as_mutable_type_pack(context.result);
        unsafe {
            (*result_mut).ty =
                crate::type_aliases::type_pack_variant::TypePackVariant::Bound(result_pack);
        }
        return true;
    }

    let mut fmt: *mut AstExprConstantString = core::ptr::null_mut();

    if !context.call_site.as_ptr().is_null() {
        let call_site = unsafe { &*context.call_site.as_ptr() };
        if call_site.func.is_null() {
            return false;
        }

        let func_node = unsafe { &*call_site.func };
        if func_node.base.class_index == AstExprIndexName::ClassIndex {
            let index_expr =
                unsafe { ast_node_as::<AstExprIndexName>(call_site.func as *mut AstNode) };
            if !index_expr.is_null() && call_site.self_ {
                let unwrapped = unwrap_group(unsafe { &mut *index_expr }.expr);
                fmt = unsafe { ast_node_as::<AstExprConstantString>(unwrapped as *mut AstNode) };
            }
        }

        if !call_site.self_ && call_site.args.size > 0 {
            fmt = unsafe {
                ast_node_as::<AstExprConstantString>(*call_site.args.data as *mut AstNode)
            };
        }
    }

    let mut format_string: Option<&str> = None;

    if !fmt.is_null() {
        let fmt_ref = unsafe { &*fmt };
        let data = fmt_ref.value.data as *const u8;
        let size = fmt_ref.value.size as usize;
        format_string = Some(unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(data, size))
        });
    } else {
        let first_arg = unsafe { *iter.operator_deref() };
        let followed = unsafe { follow_type_id(first_arg) };
        let singleton = unsafe { get_type_id::<SingletonType>(followed) };
        if !singleton.is_null() {
            if let Some(string_singleton) =
                unsafe { (*singleton).variant.get_if::<StringSingleton>() }
            {
                format_string = Some(&string_singleton.value);
            }
        }
    }

    if format_string.is_none() {
        return false;
    }

    let format_str = format_string.unwrap();
    let expected = parse_format_string(
        core::ptr::NonNull::new(unsafe { &mut *solver.builtin_types }).unwrap(),
        format_str.as_ptr() as *const core::ffi::c_char,
        format_str.len(),
    );

    let (params, tail) = flatten_type_pack_id(context.arguments);

    let param_offset = 1;

    // unify the prefix one argument at a time - needed if any of the involved types are free
    for i in 0..expected.len() {
        if i + param_offset >= params.len() {
            break;
        }
        unsafe {
            (*context.solver.as_ptr()).constraint_solver_unify(
                context.constraint.as_ptr(),
                params[i + param_offset],
                expected[i],
            );
        }
    }

    // if we know the argument count or if we have too many arguments for sure, we can issue an error
    let num_actual_params = params.len();
    let num_expected_params = expected.len() + 1; // + 1 for the format string

    if num_expected_params != num_actual_params
        && (tail.is_none() || num_expected_params < num_actual_params)
    {
        let error = TypeError::type_error_location_type_error_data(
            unsafe { &*context.call_site.as_ptr() }.base.base.location,
            crate::type_aliases::type_error_data::TypeErrorData::CountMismatch(CountMismatch {
                expected: num_expected_params,
                maximum: None,
                actual: num_actual_params,
                context: CountMismatch::Arg,
                is_variadic: tail.is_some(),
                function: String::new(),
            }),
        );
        unsafe {
            (*context.solver.as_ptr()).report_error_type_error(error);
        }
    }

    // This is invoked at solve time, so we just need to provide a type for the result of :/.format
    let result_pack = arena
        .add_type_pack_initializer_list_type_id(&[unsafe { &*solver.builtin_types }.stringType]);
    let result_mut = as_mutable_type_pack(context.result);
    unsafe {
        (*result_mut).ty =
            crate::type_aliases::type_pack_variant::TypePackVariant::Bound(result_pack);
    }

    true
}
