use crate::records::generic_type::GenericType;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ffi::c_void;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct ContainsGenerics {
    pub base: IterativeTypeVisitor,
    pub generics: *mut DenseHashSet<*const c_void>,
    pub found: bool,
}

impl ContainsGenerics {
    pub fn contains_generics(&mut self, generics: *mut DenseHashSet<*const c_void>) {
        // Keep state wiring consistent with the C++ constructor intent, but
        // avoid calling missing constructors on IterativeTypeVisitor.
        self.generics = generics;
        self.found = false;
    }

    pub fn visit_type_id(&mut self, _ty: TypeId) -> bool {
        !self.found
    }

    pub fn visit_type_id_generic_type(&mut self, ty: TypeId, _gt: &GenericType) -> bool {
        unsafe {
            let set = &*self.generics;
            let key = ty as *const c_void;
            self.found |= set.contains(&key);
        }
        true
    }

    pub fn visit_type_id_type_function_instance_type(
        &mut self,
        _ty: TypeId,
        _tfit: &TypeFunctionInstanceType,
    ) -> bool {
        !self.found
    }

    pub fn visit_type_pack_id_generic_type_pack(
        &mut self,
        tp: TypePackId,
        _gtp: &GenericTypePack,
    ) -> bool {
        unsafe {
            let set = &*self.generics;
            let key = tp as *const c_void;
            self.found |= set.contains(&key);
        }
        !self.found
    }
}
