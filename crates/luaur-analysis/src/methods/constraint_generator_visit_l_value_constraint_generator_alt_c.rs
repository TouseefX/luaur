use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::subtype_constraint::SubtypeConstraint;
use crate::records::symbol::Symbol;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn visit_l_value_scope_ptr_ast_expr_global_type_id(
        &mut self,
        scope: &ScopePtr,
        global: *mut AstExprGlobal,
        rhs_type: TypeId,
    ) {
        let global_name = unsafe { (*global).name };
        let annotated_ty = scope.lookup_symbol(Symbol::from_global(global_name));
        if let Some(annotated_ty_val) = annotated_ty {
            let def =
                unsafe { (*self.dfg).get_def(global as *const AstExprGlobal as *const AstExpr) };
            unsafe {
                *(*self.root_scope).lvalue_types.get_or_insert(def) = rhs_type;
            }

            // Ignore possible self-assignment, it doesn't create a new constraint.
            let followed_rhs = unsafe { follow_type_id(rhs_type) };
            if annotated_ty_val == followed_rhs {
                return;
            }

            let followed_annotation = unsafe { follow_type_id(annotated_ty_val) };
            let bt = unsafe { get::<BlockedType>(followed_annotation) };
            if !bt.is_null() && self.uninitialized_globals.contains(&global_name) {
                LUAU_ASSERT!(unsafe { (*bt).get_owner() }.is_null());
                self.uninitialized_globals.erase(&global_name);
                unsafe {
                    (*as_mutable_type_id(followed_annotation)).ty = TypeVariant::Bound(rhs_type);
                }
            }

            self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                unsafe { (*global).base.base.location },
                ConstraintV::Subtype(SubtypeConstraint {
                    sub_type: rhs_type,
                    super_type: annotated_ty_val,
                }),
            );
        }
    }
}
