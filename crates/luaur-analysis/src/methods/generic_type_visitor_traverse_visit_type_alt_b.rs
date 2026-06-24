//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/VisitType.h:444:generic_type_visitor_traverse`
//! Source: `Analysis/include/Luau/VisitType.h:444-510` (hand-ported)
//!
//! C++ `void GenericTypeVisitor<S>::traverse(TypePackId tp)`. Match over
//! `TypePackVariant` replaces the C++ `get<T>` if-chain (exhaustive, same
//! dispatch). NOTE: the skeleton's `TypePackVariant::Error` carries `unifiable::Error<TypePackId>`
//! variant (it dropped `Unifiable::Error`'s fields), so the error visit is
//! handed a default-constructed `ErrorTypePack` — revisit if a visitor ever
//! reads `.index`/`.synthetic` off a pack error.

use crate::records::generic_type_visitor::{GenericTypeVisitorTrait, VisitSeen};
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use core::ffi::c_void;

pub fn traverse_type_pack_id<V: GenericTypeVisitorTrait>(this: &mut V, tp: TypePackId) {
    unsafe {
        if this.visitor_base().seen.has_seen(tp as *const c_void) {
            this.cycle_type_pack_id(tp);
            return;
        }

        match &(*tp).ty {
            TypePackVariant::Bound(_) => {
                let btv = crate::functions::get_type_pack::get::<
                    crate::type_aliases::bound_type_pack::BoundTypePack,
                >(tp);
                let btv = &*btv;
                if this.visit_type_pack_id_bound_type_pack(tp, btv) {
                    this.traverse_type_pack_id(btv.boundTo);
                }
            }
            TypePackVariant::Free(ftv) => {
                this.visit_type_pack_id_free_type_pack(tp, ftv);
            }
            TypePackVariant::Generic(gtv) => {
                this.visit_type_pack_id_generic_type_pack(tp, gtv);
            }
            TypePackVariant::Error(_) => {
                let etv = ErrorTypePack {
                    index: 0,
                    synthetic: None,
                };
                this.visit_type_pack_id_error_type_pack(tp, &etv);
            }
            TypePackVariant::TypePack(pack) => {
                let res = this.visit_type_pack_id_type_pack(tp, pack);
                if res {
                    for &ty in pack.head.iter() {
                        this.traverse_type_id(ty);
                    }

                    if let Some(tail) = pack.tail {
                        this.traverse_type_pack_id(tail);
                    }
                }
            }
            TypePackVariant::Variadic(pack) => {
                let res = this.visit_type_pack_id_variadic_type_pack(tp, pack);
                if res {
                    this.traverse_type_id(pack.ty);
                }
            }
            TypePackVariant::Blocked(btp) => {
                this.visit_type_pack_id_blocked_type_pack(tp, btp);
            }
            TypePackVariant::TypeFunctionInstance(tfitp) => {
                // TypeFunctionDepthCounter tfdc{&typeFunctionDepth};
                this.visitor_base().type_function_depth += 1;

                if this.visit_type_pack_id_type_function_instance_type_pack(tp, tfitp) {
                    for &t in tfitp.typeArguments.iter() {
                        this.traverse_type_id(t);
                    }

                    for &t in tfitp.packArguments.iter() {
                        this.traverse_type_pack_id(t);
                    }
                }

                this.visitor_base().type_function_depth -= 1;
            }
        }

        this.visitor_base().seen.unsee(tp as *const c_void);
    }
}
