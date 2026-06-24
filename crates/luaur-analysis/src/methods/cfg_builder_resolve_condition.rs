//! Source: `Analysis/src/ControlFlowGraph.cpp:368-422` (hand-ported)
//! C++ `std::optional<RefinementId> CFGBuilder::resolveCondition(AstExpr* condition)`.
use crate::functions::match_type_guard::match_type_guard;
use crate::methods::refinement_arena_type_proposition::refinement_arena_type_proposition;
use crate::records::cfg_builder::CfgBuilder;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use crate::type_aliases::refinement_id_control_flow_graph::RefinementId;
use alloc::string::ToString;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_expr_unary::AstExprUnary;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl CfgBuilder {
    pub fn resolve_condition(&mut self, condition: *mut AstExpr) -> Option<RefinementId> {
        unsafe {
            // auto& arena = allocator->refinementArena;  (accessed per-use below)
            let node = condition as *mut AstNode;

            // if (auto group = condition->as<AstExprGroup>())
            //     return resolveCondition(group->expr);
            let group = ast_node_as::<AstExprGroup>(node);
            if !group.is_null() {
                return self.resolve_condition((*group).expr);
            }

            // else if (auto loc = condition->as<AstExprLocal>()) { ... }
            let loc = ast_node_as::<AstExprLocal>(node);
            if !loc.is_null() {
                // DefId def = readVariable(currentBlock, Symbol(loc->local));
                let def: DefId =
                    self.read_variable(self.current_block, Symbol::from_local((*loc).local));
                // cfg->useDefs[loc] = def;
                *self
                    .cfg
                    .as_mut()
                    .unwrap()
                    .use_defs
                    .get_or_insert(loc as *mut AstExpr) = def;
                // return arena.proposition(def, /* sense */ true);
                return Some(
                    (*self.allocator)
                        .refinement_arena
                        .proposition_def_id_bool(def, true),
                );
            }

            // else if (auto binop = condition->as<AstExprBinary>()) { ... }
            let binop = ast_node_as::<AstExprBinary>(node);
            if !binop.is_null() {
                let op = (*binop).op;
                // if (auto tg = matchTypeGuard(binop->op, binop->left, binop->right)) { ... }
                if let Some(tg) = match_type_guard(op as i32, (*binop).left, (*binop).right) {
                    // if (auto tgtLocal = tg->target->as<AstExprLocal>()) { ... }
                    let tgt_local = ast_node_as::<AstExprLocal>(tg.target() as *mut AstNode);
                    if !tgt_local.is_null() {
                        // auto def = readVariable(currentBlock, Symbol(tgtLocal->local));
                        let def = self.read_variable(
                            self.current_block,
                            Symbol::from_local((*tgt_local).local),
                        );
                        // cfg->useDefs[tgtLocal] = def;
                        *self
                            .cfg
                            .as_mut()
                            .unwrap()
                            .use_defs
                            .get_or_insert(tgt_local as *mut AstExpr) = def;
                        // bool sense = binop->op == AstExprBinary::CompareEq;
                        let sense = op == AstExprBinary::CompareEq;
                        // return arena.typeProposition(def, tg->type, tg->isTypeof, sense);
                        let arena = &mut (*self.allocator).refinement_arena;
                        return Some(refinement_arena_type_proposition(
                            arena,
                            def,
                            Some(tg.r#type().to_string()),
                            tg.isTypeof(),
                            sense,
                        ));
                    }
                    // return std::nullopt;
                    return None;
                }

                // auto lRef = resolveCondition(binop->left);
                // auto rRef = resolveCondition(binop->right);
                let l_ref = self.resolve_condition((*binop).left);
                let r_ref = self.resolve_condition((*binop).right);
                if op == AstExprBinary::And {
                    // (A and B) truthy => both truthy; a missing side still preserves the other.
                    if let (Some(l), Some(r)) = (l_ref, r_ref) {
                        return Some((*self.allocator).refinement_arena.conjunction_mut(l, r));
                    }
                    return if l_ref.is_some() { l_ref } else { r_ref };
                } else if op == AstExprBinary::Or {
                    // (A or B) truthy => at least one truthy; an unrefined side means we can't narrow.
                    if let (Some(l), Some(r)) = (l_ref, r_ref) {
                        return Some((*self.allocator).refinement_arena.disjunction_mut(l, r));
                    }
                }
                // falls through to `return std::nullopt;`
                return None;
            }

            // else if (auto unop = condition->as<AstExprUnary>()) { ... }
            let unop = ast_node_as::<AstExprUnary>(node);
            if !unop.is_null() {
                // if (unop->op == AstExprUnary::Not)
                if (*unop).op == AstExprUnary::Not {
                    // if (auto inner = resolveCondition(unop->expr))
                    //     return arena.negation(*inner);
                    if let Some(inner) = self.resolve_condition((*unop).expr) {
                        return Some((*self.allocator).refinement_arena.negation_mut(inner));
                    }
                }
            }

            // return std::nullopt;
            None
        }
    }
}
