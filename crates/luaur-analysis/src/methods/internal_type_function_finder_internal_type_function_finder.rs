use crate::functions::are_equivalent::are_equivalent;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use crate::records::internal_type_function_finder::InternalTypeFunctionFinder;
use crate::records::type_function_finder::TypeFunctionFinder;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_void;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl InternalTypeFunctionFinder {
    /// C++ `explicit InternalTypeFunctionFinder(std::vector<TypeId>& declStack)`
    /// (TypeChecker2.cpp:194), seeding `mentioned{Functions,FunctionPacks}` from a
    /// `TypeFunctionFinder` traversal of the declaration stack.
    pub fn internal_type_function_finder(decl_stack: &mut Vec<TypeId>) -> Self {
        let mut f = TypeFunctionFinder::new();
        for fn_ty in decl_stack.iter().copied() {
            f.traverse_type_id(fn_ty);
        }

        InternalTypeFunctionFinder {
            base: TypeOnceVisitor::new(String::from("InternalTypeFunctionFinder"), true),
            internal_functions: DenseHashSet::new(core::ptr::null_mut()),
            internal_pack_functions: DenseHashSet::new(core::ptr::null_mut()),
            mentioned_functions: f.mentioned_functions,
            mentioned_function_packs: f.mentioned_function_packs,
        }
    }
}

impl GenericTypeVisitorTrait for TypeFunctionFinder {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    /// TypeChecker2.cpp:171 — record every mentioned type function instance.
    fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        _tfit: &TypeFunctionInstanceType,
    ) -> bool {
        self.mentioned_functions.insert(ty);
        true
    }

    /// TypeChecker2.cpp:177 — record every mentioned type function instance pack.
    fn visit_type_pack_id_type_function_instance_type_pack(
        &mut self,
        tp: TypePackId,
        _tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        self.mentioned_function_packs.insert(tp);
        true
    }
}

impl GenericTypeVisitorTrait for InternalTypeFunctionFinder {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    /// TypeChecker2.cpp:205 — `bool visit(TypeId, const TypeFunctionInstanceType&)`.
    fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        tfit: &TypeFunctionInstanceType,
    ) -> bool {
        let mut has_generic = false;

        for &p in &tfit.type_arguments {
            unsafe {
                if !get_type_id::<GenericType>(follow_type_id(p)).is_null() {
                    has_generic = true;
                    break;
                }
            }
        }

        if !has_generic {
            for &p in &tfit.pack_arguments {
                unsafe {
                    if !get_type_pack_id::<GenericTypePack>(follow_type_pack_id(p)).is_null() {
                        has_generic = true;
                        break;
                    }
                }
            }
        }

        if has_generic {
            for mentioned in self.mentioned_functions.iter() {
                let mentioned_tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(*mentioned) };
                LUAU_ASSERT!(!mentioned_tfit.is_null());
                if are_equivalent(tfit, unsafe { &*mentioned_tfit }) {
                    return true;
                }
            }

            self.internal_functions.insert(ty);
        }

        true
    }

    /// TypeChecker2.cpp:245 — `bool visit(TypePackId, const TypeFunctionInstanceTypePack&)`.
    fn visit_type_pack_id_type_function_instance_type_pack(
        &mut self,
        tp: TypePackId,
        tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        let mut has_generic = false;

        for &p in &tfitp.typeArguments {
            unsafe {
                if !get_type_id::<GenericType>(follow_type_id(p)).is_null() {
                    has_generic = true;
                    break;
                }
            }
        }

        if !has_generic {
            for &p in &tfitp.packArguments {
                unsafe {
                    if !get_type_pack_id::<GenericTypePack>(follow_type_pack_id(p)).is_null() {
                        has_generic = true;
                        break;
                    }
                }
            }
        }

        if has_generic {
            for mentioned in self.mentioned_function_packs.iter() {
                let mentioned_tfitp =
                    unsafe { get_type_pack_id::<TypeFunctionInstanceTypePack>(*mentioned) };
                LUAU_ASSERT!(!mentioned_tfitp.is_null());
                if are_equivalent(tfitp, unsafe { &*mentioned_tfitp }) {
                    return true;
                }
            }

            self.internal_pack_functions.insert(tp);
        }

        true
    }
}
