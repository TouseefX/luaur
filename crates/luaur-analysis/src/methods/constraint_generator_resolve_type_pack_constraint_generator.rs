use crate::enums::polarity::Polarity;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl ConstraintGenerator {
    pub fn resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
        &mut self,
        scope: *mut Scope,
        tp: *mut AstTypePack,
        in_type_argument: bool,
        replace_error_with_fresh: bool,
        initial_polarity: Polarity,
    ) -> TypePackId {
        let _polarity = initial_polarity;
        self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
            scope,
            tp,
            in_type_argument,
            replace_error_with_fresh,
        )
    }
}
