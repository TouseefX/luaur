use crate::functions::follow_type::follow_type_id;
use crate::records::type_arena::TypeArena;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_ast::records::ast_expr::AstExpr;

impl TypeChecker2 {
    pub fn lookup_expected_pack(&self, expr: *mut AstExpr, arena: &mut TypeArena) -> TypePackId {
        if let Some(ty) = unsafe {
            (*self.module)
                .ast_expected_types
                .find(&(expr as *const AstExpr))
        } {
            let ty = unsafe { follow_type_id(*ty) };
            let pack = TypePack {
                head: alloc::vec![ty],
                tail: None,
            };
            let pack_var = TypePackVar {
                ty: TypePackVariant::TypePack(pack),
                persistent: false,
                owningArena: core::ptr::null_mut(),
            };
            arena.add_type_pack_type_pack(pack_var)
        } else {
            unsafe { (*self.builtin_types).anyTypePack }
        }
    }
}
