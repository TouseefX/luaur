use crate::functions::collect_operands::collect_operands;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::scope::Scope;
use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::location::Location;

impl NonStrictTypeChecker {
    pub fn will_run_time_error_function_definition(
        &mut self,
        fragment: *mut AstLocal,
        scope: *mut Scope,
        context: &NonStrictContext,
    ) -> Option<TypeId> {
        let def: DefId = unsafe { &*self.dfg }.get_def_ast_local(fragment);
        let mut defs: alloc::vec::Vec<DefId> = alloc::vec::Vec::new();
        collect_operands(def, &mut defs);

        for def_item in defs.iter() {
            if let Some(context_ty) = context.find_def_id(def_item) {
                let r1: SubtypingResult = self.subtyping.is_subtype_type_id_type_id_not_null_scope(
                    unsafe { (*self.builtin_types).unknownType },
                    context_ty,
                    scope,
                );

                let r2: SubtypingResult = self.subtyping.is_subtype_type_id_type_id_not_null_scope(
                    context_ty,
                    unsafe { (*self.builtin_types).unknownType },
                    scope,
                );

                if r1.normalization_too_complex || r2.normalization_too_complex {
                    let loc: Location = unsafe { &*fragment }.location;
                    self.report_error(
                        TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
                        &loc,
                    );
                }

                let is_unknown: bool = r1.is_subtype && r2.is_subtype;
                if is_unknown {
                    return Some(unsafe { (*self.builtin_types).unknownType });
                }
            }
        }

        None
    }
}
