use crate::functions::collect_operands::collect_operands;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::scope::Scope;
use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::FFlag;

impl NonStrictTypeChecker {
    pub fn will_run_time_error(
        &mut self,
        fragment: *mut AstExpr,
        context: &NonStrictContext,
        scope: *mut Scope,
    ) -> Option<TypeId> {
        let def: DefId = unsafe { &*self.dfg }.get_def(fragment);
        let mut defs: Vec<DefId> = Vec::new();
        collect_operands(def, &mut defs);

        for def_item in defs.iter() {
            if let Some(context_ty) = context.find_def(*def_item) {
                let actual_type = self.lookup_type(fragment);

                if self.should_skip_runtime_error_testing(actual_type) {
                    continue;
                }

                let r: SubtypingResult = self.subtyping.is_subtype_type_id_type_id_not_null_scope(
                    actual_type,
                    context_ty,
                    scope,
                );

                if r.normalization_too_complex {
                    let loc = unsafe { (*fragment).base.location };
                    self.report_error(
                        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                        &loc,
                    );
                }

                if FFlag::LuauNonStrictModeUseErrorSupressingTag.get() {
                    if r.is_subtype && !r.is_error_suppressing {
                        return Some(actual_type);
                    }
                } else {
                    if r.is_subtype {
                        return Some(actual_type);
                    }
                }
            }
        }

        None
    }
}
