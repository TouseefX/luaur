use crate::records::type_checker::TypeChecker;
use crate::records::type_pack::TypePack;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type_list::AstTypeList;

impl TypeChecker {
    pub fn resolve_type_pack_scope_ptr_ast_type_list(
        &mut self,
        scope: ScopePtr,
        types: &AstTypeList,
    ) -> TypePackId {
        if types.types.size == 0 && !types.tail_type.is_null() {
            return self
                .resolve_type_pack_scope_ptr_ast_type_pack(scope, unsafe { &*types.tail_type });
        } else if types.types.size > 0 {
            let mut head = alloc::vec::Vec::with_capacity(types.types.size as usize);
            for i in 0..types.types.size {
                let ann = unsafe { *types.types.data.add(i as usize) };
                let ty = self.resolve_type(scope.clone(), unsafe { &*ann });
                head.push(ty);
            }

            let tail = if !types.tail_type.is_null() {
                Some(
                    self.resolve_type_pack_scope_ptr_ast_type_pack(scope.clone(), unsafe {
                        &*types.tail_type
                    }),
                )
            } else {
                None
            };

            return self.add_type_pack_type_pack(TypePack { head, tail });
        }

        self.add_type_pack_type_pack(TypePack {
            head: alloc::vec::Vec::new(),
            tail: None,
        })
    }
}
