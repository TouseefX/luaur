use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn resolve_type_arguments(
        &mut self,
        scope: *mut Scope,
        type_arguments: AstArray<AstTypeOrPack>,
    ) -> (alloc::vec::Vec<TypeId>, alloc::vec::Vec<TypePackId>) {
        let mut resolved_type_arguments = alloc::vec::Vec::new();
        let mut resolved_type_pack_arguments = alloc::vec::Vec::new();

        for type_or_pack in type_arguments.iter() {
            if !type_or_pack.r#type.is_null() {
                resolved_type_arguments.push(self.resolve_type(
                    scope,
                    type_or_pack.r#type,
                    false,
                    false,
                    crate::enums::polarity::Polarity::Unknown,
                ));
            } else {
                LUAU_ASSERT!(!type_or_pack.type_pack.is_null());
                resolved_type_pack_arguments.push(
                    self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
                        scope,
                        type_or_pack.type_pack,
                        false,
                        false,
                        crate::enums::polarity::Polarity::Unknown,
                    ),
                );
            }
        }

        (resolved_type_arguments, resolved_type_pack_arguments)
    }
}
