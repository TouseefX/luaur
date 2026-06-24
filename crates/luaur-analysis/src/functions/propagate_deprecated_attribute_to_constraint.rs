//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ConstraintGenerator.cpp:1720:propagate_deprecated_attribute_to_constraint`
//! Source: `Analysis/src/ConstraintGenerator.cpp` (ConstraintGenerator.cpp:1720-1731, hand-ported)

use crate::records::generalization_constraint::GeneralizationConstraint;
use crate::type_aliases::constraint_v::{ConstraintV, ConstraintVMember};
use luaur_ast::records::ast_attr::AstAttrType;
use luaur_ast::records::ast_expr_function::AstExprFunction;

#[allow(non_snake_case)]
pub fn propagate_deprecated_attribute_to_constraint(
    c: &mut ConstraintV,
    func: *const AstExprFunction,
) {
    if let Some(genConstraint) = GeneralizationConstraint::get_if_mut(c) {
        let deprecatedAttribute = unsafe { (*func).get_attribute(AstAttrType::Deprecated) };
        genConstraint.has_deprecated_attribute = !deprecatedAttribute.is_null();
        if !deprecatedAttribute.is_null() {
            genConstraint.deprecated_info = unsafe { (*deprecatedAttribute).deprecated_info() };
        }
    }
}
