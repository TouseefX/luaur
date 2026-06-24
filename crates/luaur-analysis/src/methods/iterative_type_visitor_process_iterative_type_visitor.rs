use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::lazy_type::LazyType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::no_refine_type::NoRefineType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl IterativeTypeVisitor {
    pub fn process_type_id(&mut self, mut ty: TypeId) {
        // Morally, if `skipBoundTypes` is set, then whenever we encounter a bound
        // type we should "skip" ahead to the first non-bound type.
        //
        // We do this check here so that we treat all bound types as if they're
        // direct pointers to some final non-bound type. If we do the check later,
        // then we might get slightly different behavior depending on the exact
        // entry point for cyclic types.
        if self.skip_bound_types {
            if !unsafe { get_type_id::<BoundType>(ty) }.is_null() {
                ty = unsafe { follow_type_id(ty) };
            } else {
                let tt = unsafe { get_type_id::<TableType>(ty) };
                if !tt.is_null() && unsafe { (*tt).bound_to }.is_some() {
                    ty = unsafe { follow_type_id(ty) };
                }
            }
        }

        if self.iterative_type_visitor_has_seen(ty as *const core::ffi::c_void) {
            return;
        }

        let btv = unsafe { get_type_id::<BoundType>(ty) };
        if !btv.is_null() {
            // At this point, we know that `skipBoundTypes` is false, as
            // otherwise we would have hit the above branch.
            LUAU_ASSERT!(!self.skip_bound_types);
            if self.visit_type_id_bound_type(ty, unsafe { &*btv }) {
                self.traverse_type_id(unsafe { (*btv).boundTo });
            }
        } else if !unsafe { get_type_id::<FreeType>(ty) }.is_null() {
            let ftv = unsafe { get_type_id::<FreeType>(ty) };
            if self.visit_type_id_free_type(ty, unsafe { &*ftv }) {
                LUAU_ASSERT!(!unsafe { (*ftv).lower_bound }.is_null());
                LUAU_ASSERT!(!unsafe { (*ftv).upper_bound }.is_null());

                self.traverse_type_id(unsafe { (*ftv).lower_bound });
                self.traverse_type_id(unsafe { (*ftv).upper_bound });
            }
        } else if !unsafe { get_type_id::<GenericType>(ty) }.is_null() {
            let gtv = unsafe { get_type_id::<GenericType>(ty) };
            self.visit_type_id_generic_type(ty, unsafe { &*gtv });
        } else if !unsafe { get_type_id::<ErrorType>(ty) }.is_null() {
            let etv = unsafe { get_type_id::<ErrorType>(ty) };
            self.visit_type_id_error_type(ty, unsafe { &*etv });
        } else if !unsafe { get_type_id::<PrimitiveType>(ty) }.is_null() {
            let ptv = unsafe { get_type_id::<PrimitiveType>(ty) };
            self.visit_type_id_primitive_type(ty, unsafe { &*ptv });
        } else if !unsafe { get_type_id::<FunctionType>(ty) }.is_null() {
            let ftv = unsafe { get_type_id::<FunctionType>(ty) };
            if self.visit_type_id_function_type(ty, unsafe { &*ftv }) {
                self.traverse_type_pack_id(unsafe { (*ftv).arg_types });
                self.traverse_type_pack_id(unsafe { (*ftv).ret_types });
            }
        } else if !unsafe { get_type_id::<TableType>(ty) }.is_null() {
            let ttv = unsafe { get_type_id::<TableType>(ty) };
            // Some visitors want to see bound tables, that's why we traverse the original type
            LUAU_ASSERT!(!self.skip_bound_types || unsafe { (*ttv).bound_to }.is_none());
            if self.skip_bound_types && unsafe { (*ttv).bound_to }.is_some() {
                self.traverse_type_id(unsafe { (*ttv).bound_to }.unwrap());
            } else if self.visit_type_id_table_type(ty, unsafe { &*ttv }) {
                if let Some(bound_to) = unsafe { (*ttv).bound_to } {
                    self.traverse_type_id(bound_to);
                } else {
                    let ttv_ref = unsafe { &*ttv };
                    for (_name, prop) in ttv_ref.props.iter() {
                        if let Some(read_ty) = prop.read_ty {
                            self.traverse_type_id(read_ty);
                        }

                        // In the case that the readType and the writeType
                        // are the same pointer, just traverse once.
                        // Traversing each property twice has pretty
                        // significant performance consequences.
                        if let Some(write_ty) = prop.write_ty {
                            if !prop.is_shared() {
                                self.traverse_type_id(write_ty);
                            }
                        }
                    }

                    if let Some(indexer) = &ttv_ref.indexer {
                        self.traverse_type_id(indexer.index_type);
                        self.traverse_type_id(indexer.index_result_type);
                    }
                }
            }
        } else if !unsafe { get_type_id::<MetatableType>(ty) }.is_null() {
            let mtv = unsafe { get_type_id::<MetatableType>(ty) };
            if self.visit_type_id_metatable_type(ty, unsafe { &*mtv }) {
                self.traverse_type_id(unsafe { (*mtv).table });
                self.traverse_type_id(unsafe { (*mtv).metatable });
            }
        } else if !unsafe { get_type_id::<ExternType>(ty) }.is_null() {
            let etv = unsafe { get_type_id::<ExternType>(ty) };
            if self.visit_type_id_extern_type(ty, unsafe { &*etv }) {
                let etv_ref = unsafe { &*etv };
                for (_name, prop) in etv_ref.props.iter() {
                    if let Some(read_ty) = prop.read_ty {
                        self.traverse_type_id(read_ty);
                    }

                    // In the case that the readType and the writeType are
                    // the same pointer, just traverse once. Traversing each
                    // property twice would have pretty significant
                    // performance consequences.
                    if let Some(write_ty) = prop.write_ty {
                        if !prop.is_shared() {
                            self.traverse_type_id(write_ty);
                        }
                    }
                }

                if let Some(parent) = etv_ref.parent {
                    self.traverse_type_id(parent);
                }

                if let Some(metatable) = etv_ref.metatable {
                    self.traverse_type_id(metatable);
                }

                if let Some(indexer) = &etv_ref.indexer {
                    self.traverse_type_id(indexer.index_type);
                    self.traverse_type_id(indexer.index_result_type);
                }
            }
        } else if !unsafe { get_type_id::<AnyType>(ty) }.is_null() {
            let atv = unsafe { get_type_id::<AnyType>(ty) };
            self.visit_type_id_any_type(ty, unsafe { &*atv });
        } else if !unsafe { get_type_id::<NoRefineType>(ty) }.is_null() {
            let nrt = unsafe { get_type_id::<NoRefineType>(ty) };
            self.visit_type_id_no_refine_type(ty, unsafe { &*nrt });
        } else if !unsafe { get_type_id::<UnionType>(ty) }.is_null() {
            let utv = unsafe { get_type_id::<UnionType>(ty) };
            if self.visit_type_id_union_type(ty, unsafe { &*utv }) {
                let mut union_changed = false;
                let options = unsafe { (*utv).options.clone() };
                for opt_ty in options {
                    self.traverse_type_id(opt_ty);
                    if unsafe { get_type_id::<UnionType>(follow_type_id(ty)) }.is_null() {
                        union_changed = true;
                        break;
                    }
                }

                if union_changed {
                    self.traverse_type_id(ty);
                }
            }
        } else if !unsafe { get_type_id::<IntersectionType>(ty) }.is_null() {
            let itv = unsafe { get_type_id::<IntersectionType>(ty) };
            if self.visit_type_id_intersection_type(ty, unsafe { &*itv }) {
                let mut intersection_changed = false;
                let parts = unsafe { (*itv).parts.clone() };
                for part_ty in parts {
                    self.traverse_type_id(part_ty);
                    if unsafe { get_type_id::<IntersectionType>(follow_type_id(ty)) }.is_null() {
                        intersection_changed = true;
                        break;
                    }
                }

                if intersection_changed {
                    self.traverse_type_id(ty);
                }
            }
        } else if !unsafe { get_type_id::<LazyType>(ty) }.is_null() {
            let ltv = unsafe { get_type_id::<LazyType>(ty) };
            let unwrapped: TypeId = unsafe { (*ltv).unwrapped };
            if !unwrapped.is_null() {
                self.traverse_type_id(unwrapped);
            }

            // Visiting into LazyType that hasn't been unwrapped may necessarily
            // cause infinite expansion, so we don't do that on purpose. Asserting
            // also makes no sense, because the type _will_ happen here, most likely
            // as a property of some ExternType that doesn't need to be expanded.
        } else if !unsafe { get_type_id::<SingletonType>(ty) }.is_null() {
            let stv = unsafe { get_type_id::<SingletonType>(ty) };
            self.visit_type_id_singleton_type(ty, unsafe { &*stv });
        } else if !unsafe { get_type_id::<BlockedType>(ty) }.is_null() {
            let btv = unsafe { get_type_id::<BlockedType>(ty) };
            self.visit_type_id_blocked_type(ty, unsafe { &*btv });
        } else if !unsafe { get_type_id::<UnknownType>(ty) }.is_null() {
            let utv = unsafe { get_type_id::<UnknownType>(ty) };
            self.visit_type_id_unknown_type(ty, unsafe { &*utv });
        } else if !unsafe { get_type_id::<NeverType>(ty) }.is_null() {
            let ntv = unsafe { get_type_id::<NeverType>(ty) };
            self.visit_type_id_never_type(ty, unsafe { &*ntv });
        } else if !unsafe { get_type_id::<PendingExpansionType>(ty) }.is_null() {
            let petv = unsafe { get_type_id::<PendingExpansionType>(ty) };
            if self.visit_type_id_pending_expansion_type(ty, unsafe { &*petv }) {
                let type_arguments = unsafe { (*petv).type_arguments.clone() };
                for a in type_arguments {
                    self.traverse_type_id(a);
                }

                let pack_arguments = unsafe { (*petv).pack_arguments.clone() };
                for a in pack_arguments {
                    self.traverse_type_pack_id(a);
                }
            }
        } else if !unsafe { get_type_id::<NegationType>(ty) }.is_null() {
            let ntv = unsafe { get_type_id::<NegationType>(ty) };
            if self.visit_type_id_negation_type(ty, unsafe { &*ntv }) {
                self.traverse_type_id(unsafe { (*ntv).ty });
            }
        } else if !unsafe { get_type_id::<TypeFunctionInstanceType>(ty) }.is_null() {
            let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(ty) };
            if self.visit_type_id_type_function_instance_type(ty, unsafe { &*tfit }) {
                let type_arguments = unsafe { (*tfit).type_arguments.clone() };
                for p in type_arguments {
                    self.traverse_type_id(p);
                }

                let pack_arguments = unsafe { (*tfit).pack_arguments.clone() };
                for p in pack_arguments {
                    self.traverse_type_pack_id(p);
                }
            }
        } else {
            LUAU_ASSERT!(false /* "GenericTypeVisitor::traverse(TypeId) is not exhaustive!" */);
        }

        self.iterative_type_visitor_unsee(ty as *const core::ffi::c_void);
    }
}
