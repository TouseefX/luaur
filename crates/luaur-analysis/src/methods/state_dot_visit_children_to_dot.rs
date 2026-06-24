use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::error_type::ErrorType;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::lazy_type::LazyType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::no_refine_type::NoRefineType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::state_dot::StateDot;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::singleton_variant::SingletonVariantMember;
use crate::type_aliases::type_id::TypeId;
use luaur_common::functions::escape::escape;
use luaur_common::functions::format_append::formatAppend;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl StateDot {
    pub fn visit_children_type_id_i32(&mut self, ty: TypeId, index: i32) {
        if self.seen_ty.contains(&ty) {
            return;
        }
        self.seen_ty.insert(ty);

        self.start_node(index);
        self.start_node_label();

        unsafe {
            let t = get_type_id::<BoundType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(&mut self.result, format_args!("BoundType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();

                self.visit_child_type_id_i32_c_char(t.boundTo, index, core::ptr::null());
                return;
            }

            let t = get_type_id::<BlockedType>(ty);
            if !t.is_null() {
                formatAppend(&mut self.result, format_args!("BlockedType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<FunctionType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(&mut self.result, format_args!("FunctionType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();

                self.visit_child_type_pack_id_i32_c_char(t.arg_types, index, c"arg".as_ptr());
                self.visit_child_type_pack_id_i32_c_char(t.ret_types, index, c"ret".as_ptr());
                return;
            }

            let t = get_type_id::<TableType>(ty);
            if !t.is_null() {
                let t = &*t;
                if let Some(name) = &t.name {
                    formatAppend(&mut self.result, format_args!("TableType {}", name));
                } else if let Some(synthetic_name) = &t.synthetic_name {
                    formatAppend(
                        &mut self.result,
                        format_args!("TableType {}", synthetic_name),
                    );
                } else {
                    formatAppend(&mut self.result, format_args!("TableType {}", index));
                }
                self.finish_node_label_type_id(ty);
                self.finish_node();

                if let Some(bound_to) = t.bound_to {
                    self.visit_child_type_id_i32_c_char(bound_to, index, c"boundTo".as_ptr());
                    return;
                }

                for (name, prop) in t.props.iter() {
                    if prop.is_shared() {
                        let c_name = alloc::ffi::CString::new(name.as_bytes()).unwrap();
                        self.visit_child_type_id_i32_c_char(
                            prop.read_ty.unwrap(),
                            index,
                            c_name.as_ptr(),
                        );
                    } else {
                        if let Some(read_ty) = prop.read_ty {
                            let read_name = alloc::format!("read {}", name);
                            let c_name = alloc::ffi::CString::new(read_name.as_bytes()).unwrap();
                            self.visit_child_type_id_i32_c_char(read_ty, index, c_name.as_ptr());
                        }

                        if let Some(write_ty) = prop.write_ty {
                            let write_name = alloc::format!("write {}", name);
                            let c_name = alloc::ffi::CString::new(write_name.as_bytes()).unwrap();
                            self.visit_child_type_id_i32_c_char(write_ty, index, c_name.as_ptr());
                        }
                    }
                }
                if let Some(indexer) = &t.indexer {
                    self.visit_child_type_id_i32_c_char(
                        indexer.index_type,
                        index,
                        c"[index]".as_ptr(),
                    );
                    self.visit_child_type_id_i32_c_char(
                        indexer.index_result_type,
                        index,
                        c"[value]".as_ptr(),
                    );
                }
                for itp in &t.instantiated_type_params {
                    self.visit_child_type_id_i32_c_char(*itp, index, c"typeParam".as_ptr());
                }

                for itp in &t.instantiated_type_pack_params {
                    self.visit_child_type_pack_id_i32_c_char(
                        *itp,
                        index,
                        c"typePackParam".as_ptr(),
                    );
                }
                return;
            }

            let t = get_type_id::<MetatableType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(&mut self.result, format_args!("MetatableType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();

                self.visit_child_type_id_i32_c_char(t.table, index, c"table".as_ptr());
                self.visit_child_type_id_i32_c_char(t.metatable, index, c"metatable".as_ptr());
                return;
            }

            let t = get_type_id::<UnionType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(&mut self.result, format_args!("UnionType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();

                for opt in &t.options {
                    self.visit_child_type_id_i32_c_char(*opt, index, core::ptr::null());
                }
                return;
            }

            let t = get_type_id::<IntersectionType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(&mut self.result, format_args!("IntersectionType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();

                for part in &t.parts {
                    self.visit_child_type_id_i32_c_char(*part, index, core::ptr::null());
                }
                return;
            }

            let t = get_type_id::<LazyType>(ty);
            if !t.is_null() {
                formatAppend(&mut self.result, format_args!("LazyType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<PendingExpansionType>(ty);
            if !t.is_null() {
                formatAppend(
                    &mut self.result,
                    format_args!("PendingExpansionType {}", index),
                );
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<GenericType>(ty);
            if !t.is_null() {
                let t = &*t;
                if t.explicit_name {
                    formatAppend(&mut self.result, format_args!("GenericType {}", t.name));
                } else {
                    formatAppend(&mut self.result, format_args!("GenericType {}", index));
                }
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<FreeType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(&mut self.result, format_args!("FreeType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();

                if !t.lower_bound.is_null() && get_type_id::<NeverType>(t.lower_bound).is_null() {
                    self.visit_child_type_id_i32_c_char(
                        t.lower_bound,
                        index,
                        c"[lowerBound]".as_ptr(),
                    );
                }

                if !t.upper_bound.is_null() && get_type_id::<UnknownType>(t.upper_bound).is_null() {
                    self.visit_child_type_id_i32_c_char(
                        t.upper_bound,
                        index,
                        c"[upperBound]".as_ptr(),
                    );
                }
                return;
            }

            let t = get_type_id::<AnyType>(ty);
            if !t.is_null() {
                formatAppend(&mut self.result, format_args!("AnyType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<NoRefineType>(ty);
            if !t.is_null() {
                formatAppend(&mut self.result, format_args!("NoRefineType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<UnknownType>(ty);
            if !t.is_null() {
                formatAppend(&mut self.result, format_args!("UnknownType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<NeverType>(ty);
            if !t.is_null() {
                formatAppend(&mut self.result, format_args!("NeverType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<PrimitiveType>(ty);
            if !t.is_null() {
                let s = to_string_type_id(ty);
                formatAppend(&mut self.result, format_args!("PrimitiveType {}", s));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<ErrorType>(ty);
            if !t.is_null() {
                formatAppend(&mut self.result, format_args!("ErrorType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<ExternType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(&mut self.result, format_args!("ExternType {}", t.name));
                self.finish_node_label_type_id(ty);
                self.finish_node();

                for (name, prop) in t.props.iter() {
                    if prop.is_shared() {
                        let c_name = alloc::ffi::CString::new(name.as_bytes()).unwrap();
                        self.visit_child_type_id_i32_c_char(
                            prop.read_ty.unwrap(),
                            index,
                            c_name.as_ptr(),
                        );
                    } else {
                        if let Some(read_ty) = prop.read_ty {
                            let read_name = alloc::format!("read {}", name);
                            let c_name = alloc::ffi::CString::new(read_name.as_bytes()).unwrap();
                            self.visit_child_type_id_i32_c_char(read_ty, index, c_name.as_ptr());
                        }

                        if let Some(write_ty) = prop.write_ty {
                            let write_name = alloc::format!("write {}", name);
                            let c_name = alloc::ffi::CString::new(write_name.as_bytes()).unwrap();
                            self.visit_child_type_id_i32_c_char(write_ty, index, c_name.as_ptr());
                        }
                    }
                }

                if let Some(parent) = t.parent {
                    self.visit_child_type_id_i32_c_char(parent, index, c"[parent]".as_ptr());
                }

                if let Some(metatable) = t.metatable {
                    self.visit_child_type_id_i32_c_char(metatable, index, c"[metatable]".as_ptr());
                }

                if let Some(indexer) = &t.indexer {
                    self.visit_child_type_id_i32_c_char(
                        indexer.index_type,
                        index,
                        c"[index]".as_ptr(),
                    );
                    self.visit_child_type_id_i32_c_char(
                        indexer.index_result_type,
                        index,
                        c"[value]".as_ptr(),
                    );
                }
                return;
            }

            let t = get_type_id::<SingletonType>(ty);
            if !t.is_null() {
                let res: alloc::string::String;

                let ss = StringSingleton::get_if(&(*t).variant);
                if let Some(ss) = ss {
                    // Don't put in quotes anywhere. If it's outside of the call to escape,
                    // then it's invalid syntax. If it's inside, then escaping is super noisy.
                    res = alloc::format!("string: {}", escape(&ss.value, false));
                } else if let Some(bs) = BooleanSingleton::get_if(&(*t).variant) {
                    res = alloc::format!("boolean: {}", if bs.value { "true" } else { "false" });
                } else {
                    LUAU_ASSERT!(false);
                    res = alloc::string::String::new();
                }

                formatAppend(&mut self.result, format_args!("SingletonType {}", res));
                self.finish_node_label_type_id(ty);
                self.finish_node();
                return;
            }

            let t = get_type_id::<NegationType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(&mut self.result, format_args!("NegationType {}", index));
                self.finish_node_label_type_id(ty);
                self.finish_node();

                self.visit_child_type_id_i32_c_char(t.ty, index, c"[negated]".as_ptr());
                return;
            }

            let t = get_type_id::<TypeFunctionInstanceType>(ty);
            if !t.is_null() {
                let t = &*t;
                formatAppend(
                    &mut self.result,
                    format_args!(
                        "TypeFunctionInstanceType {} {}",
                        t.function.as_ref().name,
                        index
                    ),
                );
                self.finish_node_label_type_id(ty);
                self.finish_node();

                for ty_param in &t.type_arguments {
                    self.visit_child_type_id_i32_c_char(*ty_param, index, core::ptr::null());
                }

                for tp_param in &t.pack_arguments {
                    self.visit_child_type_pack_id_i32_c_char(*tp_param, index, core::ptr::null());
                }
                return;
            }

            // unknown type kind
            LUAU_ASSERT!(false);
        }
    }
}
