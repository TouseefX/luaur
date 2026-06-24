use crate::functions::follow_type::follow_type_id;
use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker {
    pub fn un_type_pack(
        &mut self,
        scope: &ScopePtr,
        tp: TypePackId,
        expected_length: usize,
        location: &Location,
    ) -> alloc::vec::Vec<TypeId> {
        let expected_type_pack = self.add_type_pack_type_pack(TypePack {
            head: alloc::vec::Vec::new(),
            tail: None,
        });
        let expected_pack = unsafe {
            crate::functions::get_mutable_type_pack::get_mutable_type_pack_id::<
                crate::records::type_pack::TypePack,
            >(expected_type_pack)
        };
        LUAU_ASSERT!(!expected_pack.is_null());
        for _ in 0..expected_length {
            unsafe { &mut *expected_pack }
                .head
                .push(self.fresh_type_scope_ptr(scope.clone()));
        }
        let old_errors_size = unsafe { (*self.current_module.as_ref().unwrap()).errors.len() };
        self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
            tp,
            expected_type_pack,
            scope,
            location,
            crate::records::count_mismatch::CountMismatchContext::Arg,
        );
        unsafe {
            (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module))
                .errors
                .truncate(old_errors_size)
        };
        let mut result = unsafe { &mut *expected_pack }.head.clone();
        for ty in &mut result {
            *ty = unsafe { follow_type_id(*ty) };
        }
        result
    }
}
