use crate::functions::as_mutable_type_pack_alt_d::as_mutable_type_pack;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::records::generic_error::GenericError;
use crate::records::magic_function_call_context::MagicFunctionCallContext;
use crate::records::type_error::TypeError;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use alloc::string::ToString;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

pub fn magic_select_infer(context: &MagicFunctionCallContext) -> bool {
    let solver = unsafe { context.solver.as_ref() };
    let call_site = unsafe { context.call_site.as_ref() };

    if call_site.args.size <= 0 {
        let error = TypeError::type_error_location_type_error_data(
            call_site.base.base.location,
            TypeErrorData::GenericError(GenericError::new(
                "select should take 1 or more arguments".to_string(),
            )),
        );
        unsafe {
            (*context.solver.as_ptr()).report_error_type_error(error);
        }
        return false;
    }

    let arg1 = unsafe { *call_site.args.data.add(0) };

    let num = unsafe { ast_node_as::<AstExprConstantNumber>(arg1 as *mut AstNode) };
    if !num.is_null() {
        let (v, tail) = flatten_type_pack_id(context.arguments);

        let offset = unsafe { (*num).value } as i32;
        if offset > 0 {
            let offset_usize = offset as usize;
            if offset_usize < v.len() {
                let res: Vec<TypeId> = v[offset_usize..].to_vec();
                let res_type_pack = unsafe { &mut *solver.arena }
                    .add_type_pack_vector_type_id_optional_type_pack_id(res, tail);
                let result_mut = as_mutable_type_pack(context.result);
                unsafe {
                    (*result_mut).ty = TypePackVariant::Bound(res_type_pack);
                }
            } else if let Some(tail) = tail {
                let result_mut = as_mutable_type_pack(context.result);
                unsafe {
                    (*result_mut).ty = TypePackVariant::Bound(tail);
                }
            }

            return true;
        }

        return false;
    }

    let str_expr = unsafe { ast_node_as::<AstExprConstantString>(arg1 as *mut AstNode) };
    if !str_expr.is_null()
        && unsafe { (*str_expr).value.size } == 1
        && unsafe { *(*str_expr).value.data } == b'#' as core::ffi::c_char
    {
        let number_type_pack = unsafe { &mut *solver.arena }
            .add_type_pack_initializer_list_type_id(
                &[unsafe { &*solver.builtin_types }.numberType],
            );
        let result_mut = as_mutable_type_pack(context.result);
        unsafe {
            (*result_mut).ty = TypePackVariant::Bound(number_type_pack);
        }
        return true;
    }

    false
}
