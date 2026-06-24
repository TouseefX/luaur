//! Faithful port of `copyError` (Analysis/src/Error.cpp:1463-1705).
//!
//! The C++ original is a function template whose body is a chain of
//! `if constexpr (std::is_same_v<T, ...>)` branches, so the active branch is
//! selected at compile time from the concrete error type `T` the caller
//! instantiates it with. The faithful Rust equivalent of that compile-time
//! type switch is a trait: `copy_error<T>` forwards to `T`'s `CopyError`
//! implementation, and each branch of the original `if constexpr` cascade
//! becomes one `impl CopyError for <ErrorType>` whose body is exactly that
//! branch (empty for the error kinds that hold no `TypeId`/`TypePackId`).
//!
//! `::Luau::clone(ty, destArena, cloneState)` is an overload set keyed on the
//! argument type; the ports preserve that as three separate free functions
//! (`TypeId`, `TypePackId`, `TypeFun`), wrapped below as `clone_type`,
//! `clone_pack` and `clone_type_fun`.

use crate::records::clone_state::CloneState;
use crate::records::type_arena::TypeArena;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

/// `clone(TypeId, ...)` overload.
fn clone_type(ty: TypeId, dest_arena: &mut TypeArena, clone_state: &mut CloneState) -> TypeId {
    crate::functions::clone_clone_alt_b::clone(ty, dest_arena, clone_state)
}

/// `clone(TypePackId, ...)` overload.
fn clone_pack(
    tp: TypePackId,
    dest_arena: &mut TypeArena,
    clone_state: &mut CloneState,
) -> TypePackId {
    crate::functions::clone_clone::clone(tp, dest_arena, clone_state)
}

/// `clone(TypeFun, ...)` overload.
fn clone_type_fun(
    tf: &TypeFun,
    dest_arena: &mut TypeArena,
    clone_state: &mut CloneState,
) -> TypeFun {
    crate::functions::clone_clone_alt_c::clone(tf, dest_arena, clone_state)
}

/// Faithful realization of the `if constexpr` type switch in `copyError`.
///
/// One `impl` per error kind, each carrying exactly the branch body from the
/// C++ source.
pub trait CopyError {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState);
}

/// `template<typename T> void copyError(T& e, TypeArena& destArena, CloneState& cloneState)`.
pub fn copy_error<T: CopyError>(
    e: &mut T,
    dest_arena: &mut TypeArena,
    clone_state: &mut CloneState,
) {
    e.copy_error_impl(dest_arena, clone_state);
}

/// Re-dispatch over a nested `TypeErrorData`, mirroring the C++
/// `visit(visitErrorData, e.error->data)` recursion. Selects the concrete
/// branch for the active variant and forwards to its `CopyError` impl.
fn visit_error_data(
    data: &mut TypeErrorData,
    dest_arena: &mut TypeArena,
    clone_state: &mut CloneState,
) {
    match data {
        TypeErrorData::TypeMismatch(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnknownSymbol(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnknownProperty(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::NotATable(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CannotExtendTable(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CannotCompareUnrelatedTypes(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::OnlyTablesCanHaveMethods(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::DuplicateTypeDefinition(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CountMismatch(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::FunctionDoesNotTakeSelf(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::FunctionRequiresSelf(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::OccursCheckFailed(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnknownRequire(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::IncorrectGenericParameterCount(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::SyntaxError(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CodeTooComplex(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnificationTooComplex(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnknownPropButFoundLikeProp(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::GenericError(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::InternalError(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::ConstraintSolvingIncompleteError(e) => {
            copy_error(e, dest_arena, clone_state)
        }
        TypeErrorData::CannotCallNonFunction(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::ExtraInformation(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::DeprecatedApiUsed(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::ModuleHasCyclicDependency(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::IllegalRequire(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::FunctionExitsWithoutReturning(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::DuplicateGenericParameter(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CannotAssignToNever(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CannotInferBinaryOperation(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::MissingProperties(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::SwappedGenericTypeParameter(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::OptionalValueAccess(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::MissingUnionProperty(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::TypesAreUnrelated(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::NormalizationTooComplex(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::TypePackMismatch(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::DynamicPropertyLookupOnExternTypesUnsafe(e) => {
            copy_error(e, dest_arena, clone_state)
        }
        TypeErrorData::UninhabitedTypeFunction(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UninhabitedTypePackFunction(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::WhereClauseNeeded(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::PackWhereClauseNeeded(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CheckedFunctionCallError(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::NonStrictFunctionDefinitionError(e) => {
            copy_error(e, dest_arena, clone_state)
        }
        TypeErrorData::PropertyAccessViolation(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CheckedFunctionIncorrectArgs(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnexpectedTypeInSubtyping(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnexpectedTypePackInSubtyping(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::ExplicitFunctionAnnotationRecommended(e) => {
            copy_error(e, dest_arena, clone_state)
        }
        TypeErrorData::UserDefinedTypeFunctionError(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::BuiltInTypeFunctionError(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::ReservedIdentifier(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnexpectedArrayLikeTableItem(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::CannotCheckDynamicStringFormatCalls(e) => {
            copy_error(e, dest_arena, clone_state)
        }
        TypeErrorData::GenericTypeCountMismatch(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::GenericTypePackCountMismatch(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::MultipleNonviableOverloads(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::RecursiveRestraintViolation(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::GenericBoundsMismatch(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::UnappliedTypeFunction(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::InstantiateGenericsOnNonFunction(e) => {
            copy_error(e, dest_arena, clone_state)
        }
        TypeErrorData::TypeInstantiationCountMismatch(e) => copy_error(e, dest_arena, clone_state),
        TypeErrorData::AmbiguousFunctionCall(e) => copy_error(e, dest_arena, clone_state),
    }
}

// ---------------------------------------------------------------------------
// One impl per `if constexpr` branch of `copyError`.
// ---------------------------------------------------------------------------

impl CopyError for crate::records::type_mismatch::TypeMismatch {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.wanted_type = clone_type(self.wanted_type, dest_arena, clone_state);
        self.given_type = clone_type(self.given_type, dest_arena, clone_state);

        if let Some(error) = self.error.as_mut() {
            visit_error_data(
                &mut alloc::sync::Arc::make_mut(error).data,
                dest_arena,
                clone_state,
            );
        }
    }
}

impl CopyError for crate::records::unknown_symbol::UnknownSymbol {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::unknown_property::UnknownProperty {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.table = clone_type(self.table, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::not_a_table::NotATable {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.ty = clone_type(self.ty, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::cannot_extend_table::CannotExtendTable {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.table_type = clone_type(self.table_type, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::cannot_compare_unrelated_types::CannotCompareUnrelatedTypes {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.left = clone_type(self.left, dest_arena, clone_state);
        self.right = clone_type(self.right, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::only_tables_can_have_methods::OnlyTablesCanHaveMethods {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.table_type = clone_type(self.table_type, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::duplicate_type_definition::DuplicateTypeDefinition {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::count_mismatch::CountMismatch {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::function_does_not_take_self::FunctionDoesNotTakeSelf {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::function_requires_self::FunctionRequiresSelf {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::occurs_check_failed::OccursCheckFailed {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::unknown_require::UnknownRequire {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError
    for crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount
{
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.type_fun = clone_type_fun(&self.type_fun, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::syntax_error::SyntaxError {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::code_too_complex::CodeTooComplex {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::unification_too_complex::UnificationTooComplex {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.table = clone_type(self.table, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::generic_error::GenericError {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::internal_error::InternalError {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError
    for crate::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError
{
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::cannot_call_non_function::CannotCallNonFunction {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.ty = clone_type(self.ty, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::extra_information::ExtraInformation {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::deprecated_api_used::DeprecatedApiUsed {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::module_has_cyclic_dependency::ModuleHasCyclicDependency {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::illegal_require::IllegalRequire {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::function_exits_without_returning::FunctionExitsWithoutReturning {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.expected_return_type = clone_pack(self.expected_return_type, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::duplicate_generic_parameter::DuplicateGenericParameter {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::cannot_infer_binary_operation::CannotInferBinaryOperation {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::missing_properties::MissingProperties {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.super_type = clone_type(self.super_type, dest_arena, clone_state);
        self.sub_type = clone_type(self.sub_type, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::swapped_generic_type_parameter::SwappedGenericTypeParameter {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::optional_value_access::OptionalValueAccess {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.optional = clone_type(self.optional, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::missing_union_property::MissingUnionProperty {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.r#type = clone_type(self.r#type, dest_arena, clone_state);

        for ty in self.missing.iter_mut() {
            *ty = clone_type(*ty, dest_arena, clone_state);
        }
    }
}

impl CopyError for crate::records::types_are_unrelated::TypesAreUnrelated {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.left = clone_type(self.left, dest_arena, clone_state);
        self.right = clone_type(self.right, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::normalization_too_complex::NormalizationTooComplex {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::type_pack_mismatch::TypePackMismatch {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.wanted_tp = clone_pack(self.wanted_tp, dest_arena, clone_state);
        self.given_tp = clone_pack(self.given_tp, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.ty = clone_type(self.ty, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::uninhabited_type_function::UninhabitedTypeFunction {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.ty = clone_type(self.ty, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.recommended_return = clone_type(self.recommended_return, dest_arena, clone_state);
        for (_, t) in self.recommended_args.iter_mut() {
            *t = clone_type(*t, dest_arena, clone_state);
        }
    }
}

impl CopyError for crate::records::uninhabited_type_pack_function::UninhabitedTypePackFunction {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.tp = clone_pack(self.tp, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::where_clause_needed::WhereClauseNeeded {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.ty = clone_type(self.ty, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::pack_where_clause_needed::PackWhereClauseNeeded {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.tp = clone_pack(self.tp, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::checked_function_call_error::CheckedFunctionCallError {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.expected = clone_type(self.expected, dest_arena, clone_state);
        self.passed = clone_type(self.passed, dest_arena, clone_state);
    }
}

impl CopyError
    for crate::records::non_strict_function_definition_error::NonStrictFunctionDefinitionError
{
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.argument_type = clone_type(self.argument_type, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::property_access_violation::PropertyAccessViolation {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.table = clone_type(self.table, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::checked_function_incorrect_args::CheckedFunctionIncorrectArgs {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::unexpected_type_in_subtyping::UnexpectedTypeInSubtyping {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.ty = clone_type(self.ty, dest_arena, clone_state);
    }
}

impl CopyError
    for crate::records::unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping
{
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.tp = clone_pack(self.tp, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::user_defined_type_function_error::UserDefinedTypeFunctionError {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::built_in_type_function_error::BuiltInTypeFunctionError {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::cannot_assign_to_never::CannotAssignToNever {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.rhsType = clone_type(self.rhsType, dest_arena, clone_state);

        for ty in self.cause.iter_mut() {
            *ty = clone_type(*ty, dest_arena, clone_state);
        }
    }
}

impl CopyError for crate::records::unexpected_array_like_table_item::UnexpectedArrayLikeTableItem {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::reserved_identifier::ReservedIdentifier {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::generic_type_count_mismatch::GenericTypeCountMismatch {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::generic_type_pack_count_mismatch::GenericTypePackCountMismatch {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::multiple_nonviable_overloads::MultipleNonviableOverloads {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::recursive_restraint_violation::RecursiveRestraintViolation {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::generic_bounds_mismatch::GenericBoundsMismatch {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        for lower_bound in self.lower_bounds.iter_mut() {
            *lower_bound = clone_type(*lower_bound, dest_arena, clone_state);
        }
        for upper_bound in self.upper_bounds.iter_mut() {
            *upper_bound = clone_type(*upper_bound, dest_arena, clone_state);
        }
    }
}

impl CopyError
    for crate::records::instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction
{
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError
    for crate::records::type_instantiation_count_mismatch::TypeInstantiationCountMismatch
{
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.functionType = clone_type(self.functionType, dest_arena, clone_state);
    }
}

impl CopyError for crate::records::unapplied_type_function::UnappliedTypeFunction {
    fn copy_error_impl(&mut self, _dest_arena: &mut TypeArena, _clone_state: &mut CloneState) {}
}

impl CopyError for crate::records::ambiguous_function_call::AmbiguousFunctionCall {
    fn copy_error_impl(&mut self, dest_arena: &mut TypeArena, clone_state: &mut CloneState) {
        self.function = clone_type(self.function, dest_arena, clone_state);
        self.arguments = clone_pack(self.arguments, dest_arena, clone_state);
    }
}
