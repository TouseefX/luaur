use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type_list::AstTypeList;

impl ConstraintGenerator {
    pub fn resolve_type_pack_scope_ptr_ast_type_list_bool_bool(
        &mut self,
        _scope: *mut Scope,
        _list: &AstTypeList,
        _in_type_arguments: bool,
        _replace_error_with_fresh: bool,
    ) -> TypePackId {
        let mut head = alloc::vec::Vec::new();
        for head_ty in _list.types.iter() {
            head.push(self.resolve_type_constraint_generator_alt_b(
                _scope,
                *head_ty,
                _in_type_arguments,
                _replace_error_with_fresh,
            ));
        }
        let tail = if !_list.tail_type.is_null() {
            Some(self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
                _scope,
                _list.tail_type,
                _in_type_arguments,
                _replace_error_with_fresh,
            ))
        } else {
            None
        };
        self.add_type_pack(head, tail)
    }
}
