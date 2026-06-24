//! Source: `Analysis/src/ConstraintGenerator.cpp:3223-3279` (hand-ported)
//! C++ `Inference ConstraintGenerator::checkIndexName(scope, key, indexee, index, indexLocation)`.
use crate::enums::value_context::ValueContext;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::in_conditional::in_conditional;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::has_prop_constraint::HasPropConstraint;
use crate::records::inference::Inference;
use crate::records::refinement_key::RefinementKey;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn check_index_name(
        &mut self,
        scope: &ScopePtr,
        key: *const RefinementKey,
        indexee: *mut AstExpr,
        index: &String,
        index_location: Location,
    ) -> Inference {
        unsafe {
            let obj = self.check_scope_ptr_ast_expr(scope, indexee).ty;
            let mut result: TypeId = core::ptr::null();

            // We optimize away the HasProp constraint in simple cases so that we can
            // reason about updates to unsealed tables more accurately.

            let mut tt = get_table_type(obj);

            // This is a little bit iffy but I *believe* it is okay because, if the
            // local's domain is going to be extended at all, it will be someplace after
            // the current lexical position within the script.
            if tt.is_none() {
                if let Some(local_domain) = self.local_types.find(&obj) {
                    if local_domain.size() == 1 {
                        let first = local_domain.order[0];
                        tt = get_table_type(first);
                    }
                }
            }

            if let Some(tt) = tt {
                if let Some(prop) = tt.props.get(index) {
                    if let Some(read_ty) = prop.read_ty {
                        result = read_ty;
                    }
                }
            }

            if let Some(cached_has_prop_result) =
                self.prop_index_pairs_seen.find(&(obj, index.clone()))
            {
                result = *cached_has_prop_result;
            }

            if result.is_null() {
                result = (*self.arena).add_type(BlockedType::default());

                let c = self.add_constraint_scope_ptr_location_constraint_v(
                    scope,
                    (*indexee).base.location,
                    ConstraintV::HasProp(HasPropConstraint {
                        result_type: result,
                        subject_type: obj,
                        prop: index.clone(),
                        context: ValueContext::RValue,
                        in_conditional: in_conditional(self.type_context),
                        suppress_simplification: false,
                    }),
                );
                let blocked = get_mutable_type_id::<BlockedType>(result);
                (*blocked).set_owner(c as *const _);
                *self
                    .prop_index_pairs_seen
                    .get_or_insert((obj, index.clone())) = result;
            }

            if !key.is_null() {
                if let Some(ty) = self.lookup(
                    scope,
                    index_location,
                    (*key).def as crate::type_aliases::def_id_def::DefId,
                    false,
                ) {
                    let refinement = self
                        .refinement_arena
                        .proposition_refinement_key_type_id(key, (*self.builtin_types).truthyType);
                    return Inference::inference_type_id_refinement_id(ty, refinement);
                }

                self.update_r_value_refinements_scope_ptr_def_id_type_id(
                    scope,
                    (*key).def as crate::type_aliases::def_id_def::DefId,
                    result,
                );
            }

            if !key.is_null() {
                let refinement = self
                    .refinement_arena
                    .proposition_refinement_key_type_id(key, (*self.builtin_types).truthyType);
                Inference::inference_type_id_refinement_id(result, refinement)
            } else {
                Inference::inference_type_id_refinement_id(result, core::ptr::null_mut())
            }
        }
    }
}
