use crate::records::type_arena::TypeArena;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;

impl TypeChecker2 {
    pub fn reconstruct_pack(
        &mut self,
        exprs: AstArray<*mut AstExpr>,
        arena: &mut TypeArena,
    ) -> TypePackId {
        if exprs.size == 0 {
            let tp = TypePackVar {
                ty: TypePackVariant::TypePack(TypePack {
                    head: alloc::vec::Vec::new(),
                    tail: None,
                }),
                persistent: false,
                owningArena: core::ptr::null_mut(),
            };
            return arena.add_type_pack_type_pack(tp);
        }

        let mut head: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();

        for i in 0..(exprs.size - 1) {
            let ty = self.lookup_type(unsafe { *exprs.data.add(i) });
            head.push(ty);
        }

        let tail = self.lookup_pack(unsafe { *exprs.data.add(exprs.size - 1) });
        let tp = TypePackVar {
            ty: TypePackVariant::TypePack(TypePack {
                head,
                tail: Some(tail),
            }),
            persistent: false,
            owningArena: core::ptr::null_mut(),
        };
        arena.add_type_pack_type_pack(tp)
    }
}
