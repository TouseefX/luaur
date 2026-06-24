use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl IterativeTypeVisitor {
    pub fn process_type_pack_id(&mut self, tp: TypePackId) {
        if self.iterative_type_visitor_has_seen(tp as *const core::ffi::c_void) {
            return;
        }

        let btv = unsafe { get_type_pack_id::<BoundTypePack>(tp) };
        if !btv.is_null() {
            if self.visit_type_pack_id_bound_type_pack(tp, unsafe { &*btv }) {
                self.traverse_type_pack_id(unsafe { (*btv).boundTo });
            }
        } else if !unsafe { get_type_pack_id::<FreeTypePack>(tp) }.is_null() {
            let ftv = unsafe { get_type_pack_id::<FreeTypePack>(tp) };
            self.visit_type_pack_id_free_type_pack(tp, unsafe { &*ftv });
        } else if !unsafe { get_type_pack_id::<GenericTypePack>(tp) }.is_null() {
            let gtv = unsafe { get_type_pack_id::<GenericTypePack>(tp) };
            self.visit_type_pack_id_generic_type_pack(tp, unsafe { &*gtv });
        } else if !unsafe { get_type_pack_id::<ErrorTypePack>(tp) }.is_null() {
            let etv = unsafe { get_type_pack_id::<ErrorTypePack>(tp) };
            self.visit_type_pack_id_error_type_pack(tp, unsafe { &*etv });
        } else if !unsafe { get_type_pack_id::<TypePack>(tp) }.is_null() {
            let pack = unsafe { get_type_pack_id::<TypePack>(tp) };
            let res = self.visit_type_pack_id_type_pack(tp, unsafe { &*pack });
            if res {
                let head = unsafe { (*pack).head.clone() };
                for ty in head {
                    self.traverse_type_id(ty);
                }

                if let Some(tail) = unsafe { (*pack).tail } {
                    self.traverse_type_pack_id(tail);
                }
            }
        } else if !unsafe { get_type_pack_id::<VariadicTypePack>(tp) }.is_null() {
            let pack = unsafe { get_type_pack_id::<VariadicTypePack>(tp) };
            let res = self.visit_type_pack_id_variadic_type_pack(tp, unsafe { &*pack });
            if res {
                self.traverse_type_id(unsafe { (*pack).ty });
            }
        } else if !unsafe { get_type_pack_id::<BlockedTypePack>(tp) }.is_null() {
            let btp = unsafe { get_type_pack_id::<BlockedTypePack>(tp) };
            self.visit_type_pack_id_blocked_type_pack(tp, unsafe { &*btp });
        } else if !unsafe { get_type_pack_id::<TypeFunctionInstanceTypePack>(tp) }.is_null() {
            let tfitp = unsafe { get_type_pack_id::<TypeFunctionInstanceTypePack>(tp) };
            if self.visit_type_pack_id_type_function_instance_type_pack(tp, unsafe { &*tfitp }) {
                let type_arguments = unsafe { (*tfitp).typeArguments.clone() };
                for t in type_arguments {
                    self.traverse_type_id(t);
                }

                let pack_arguments = unsafe { (*tfitp).packArguments.clone() };
                for t in pack_arguments {
                    self.traverse_type_pack_id(t);
                }
            }
        } else {
            LUAU_ASSERT!(false /* "GenericTypeVisitor::traverse(TypePackId) is not exhaustive!" */);
        }

        self.iterative_type_visitor_unsee(tp as *const core::ffi::c_void);
    }
}
