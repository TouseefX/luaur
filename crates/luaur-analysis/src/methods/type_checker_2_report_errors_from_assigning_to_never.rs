use crate::enums::normalization_result::NormalizationResult;
use crate::enums::reason::Reason;
use crate::enums::value_context::ValueContext;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::cannot_assign_to_never::CannotAssignToNever;
use crate::records::never_type::NeverType;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_error::TypeError;
use crate::type_aliases::type_error_data::IntoTypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeChecker2 {
    pub fn report_errors_from_assigning_to_never(&mut self, lhs: *mut AstExpr, rhs_type: TypeId) {
        unsafe {
            let index_name = luaur_ast::rtti::ast_node_as::<AstExprIndexName>(
                lhs as *mut luaur_ast::records::ast_node::AstNode,
            );
            if index_name.is_null() {
                return;
            }

            let indexed_type = self.lookup_type((*index_name).expr);

            // if it's already never, I don't think we have anything to do here.
            if !get_type_id::<NeverType>(indexed_type).is_null() {
                return;
            }

            let _prop = core::ffi::CStr::from_ptr((*index_name).index.value)
                .to_string_lossy()
                .into_owned();

            // C++: `std::shared_ptr<const NormalizedType> norm =
            // normalizer.normalize(indexedType);` followed by `if (!norm) { reportError(
            // NormalizationTooComplex{}); return; }`. `Normalizer::normalize` here returns a
            // non-nullable `Arc<NormalizedType>`, so the limits-exceeded null case the C++
            // guards against is not representable through this signature and the value is
            // always present.
            let norm = self.normalizer.normalize(indexed_type);

            // if the type is error suppressing, we don't actually have any work left to do.
            if norm.should_suppress_errors() {
                return;
            }

            let location = (*lhs).base.location;
            let mut cause = alloc::vec::Vec::new();

            // C++ passes `lookupProp(...).typesOfProp` as the cause.  The full
            // `lookup_prop` method is still a stub, but the table component path
            // is enough to preserve the tagged-union narrowing reason here.
            for &ty in norm.tables.order.iter() {
                if self.normalizer.is_inhabited_type_id(ty) != NormalizationResult::True {
                    continue;
                }

                let mut seen = DenseHashSet::new(core::ptr::null::<crate::records::r#type::Type>());
                let mut dummy_errors: alloc::vec::Vec<TypeError> = alloc::vec::Vec::new();
                let prop_type = self.has_index_type_from_type(
                    ty,
                    &_prop,
                    ValueContext::LValue,
                    &location,
                    &mut seen,
                    (*self.builtin_types).stringType,
                    &mut dummy_errors,
                );

                if prop_type.present == NormalizationResult::True {
                    if let Some(result) = prop_type.result {
                        cause.push(result);
                    }
                }
            }

            let err = CannotAssignToNever::new(rhs_type, cause, Reason::PropertyNarrowed);
            self.report_error_type_error_data_location(err.into_type_error_data(), &location);
        }
    }
}
