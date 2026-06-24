//! @interface-stub
use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use luaur_ast::records::ast_stat_return::AstStatReturn;

impl ExpectedTypeVisitor {
    pub fn visit_ast_stat_return(&mut self, stat: *mut AstStatReturn) -> bool {
        unsafe {
            let stat_ref = &*stat;

            let scope =
                (*self.root_scope).find_narrowest_scope_containing(stat_ref.base.base.location);

            let mut it = begin_type_pack_id((*scope).return_type);
            let end_it = end_type_pack_id((*scope).return_type);
            let mut idx: usize = 0;

            while idx < stat_ref.list.size && !it.operator_eq(&end_it) {
                let expr = *stat_ref.list.data.add(idx);
                self.apply_expected_type(*it.operator_deref(), expr as *const _);
                it.operator_inc();
                idx += 1;
            }
        }

        true
    }
}
