use crate::functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id;
use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::records::type_function_type_pack::TypeFunctionTypePack;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl IterativeTypeFunctionTypeVisitor {
    pub fn process_type_function_type_pack_id(&mut self, tp: TypeFunctionTypePackId) {
        if self.has_seen(tp as *const core::ffi::c_void) {
            return;
        }

        let tftp = get_type_function_type_pack_id::<TypeFunctionTypePack>(tp);
        let tfvtp = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp);
        let tfgtv = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp);

        if !tftp.is_null() {
            if self.visit_type_function_type_pack_id_type_function_type_pack(tp, unsafe { &*tftp })
            {
                let head = unsafe { (*tftp).head.clone() };
                for ty in head {
                    self.traverse_type_function_type_id(ty);
                }

                if let Some(tail) = unsafe { (*tftp).tail } {
                    self.traverse_type_function_type_pack_id(tail);
                }
            }
        } else if !tfvtp.is_null() {
            if self.visit_type_function_type_pack_id_type_function_variadic_type_pack(tp, unsafe {
                &*tfvtp
            }) {
                let inner = unsafe { (*tfvtp).type_id };
                self.traverse_type_function_type_id(inner);
            }
        } else if !tfgtv.is_null() {
            self.visit_type_function_type_pack_id_type_function_generic_type_pack(tp, unsafe {
                &*tfgtv
            });
        } else {
            LUAU_ASSERT!(
                false /* "GenericTypeFunctionTypeVisitor::traverse(TypeFunctionTypePackId) is not exhaustive!" */
            );
        }

        self.unsee(tp as *const core::ffi::c_void);
    }
}
