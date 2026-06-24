use alloc::string::String;
use core::fmt::Write;

use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::functions::to_string_to_string_alt_d::to_string_type_pack_id;
use crate::functions::to_string_type_function_error::to_string_type_function_error;
use luaur_ast::functions::to_string_ast_alt_b::to_string_ast_expr_binary_op;

use crate::records::ambiguous_function_call::AmbiguousFunctionCall;
use crate::records::built_in_type_function_error::BuiltInTypeFunctionError;
use crate::records::cannot_assign_to_never::CannotAssignToNever;
use crate::records::cannot_call_non_function::CannotCallNonFunction;
use crate::records::cannot_check_dynamic_string_format_calls::CannotCheckDynamicStringFormatCalls;
use crate::records::cannot_compare_unrelated_types::CannotCompareUnrelatedTypes;
use crate::records::cannot_extend_table::CannotExtendTable;
use crate::records::cannot_infer_binary_operation::CannotInferBinaryOperation;
use crate::records::checked_function_call_error::CheckedFunctionCallError;
use crate::records::checked_function_incorrect_args::CheckedFunctionIncorrectArgs;
use crate::records::code_too_complex::CodeTooComplex;
use crate::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
use crate::records::count_mismatch::CountMismatch;
use crate::records::deprecated_api_used::DeprecatedApiUsed;
use crate::records::duplicate_generic_parameter::DuplicateGenericParameter;
use crate::records::duplicate_type_definition::DuplicateTypeDefinition;
use crate::records::dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe;
use crate::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;
use crate::records::extra_information::ExtraInformation;
use crate::records::function_does_not_take_self::FunctionDoesNotTakeSelf;
use crate::records::function_exits_without_returning::FunctionExitsWithoutReturning;
use crate::records::function_requires_self::FunctionRequiresSelf;
use crate::records::generic_bounds_mismatch::GenericBoundsMismatch;
use crate::records::generic_error::GenericError;
use crate::records::generic_type_count_mismatch::GenericTypeCountMismatch;
use crate::records::generic_type_pack_count_mismatch::GenericTypePackCountMismatch;
use crate::records::illegal_require::IllegalRequire;
use crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;
use crate::records::instantiate_generics_on_non_function::InstantiateGenericsOnNonFunction;
use crate::records::internal_error::InternalError;
use crate::records::missing_properties::MissingProperties;
use crate::records::missing_union_property::MissingUnionProperty;
use crate::records::module_has_cyclic_dependency::ModuleHasCyclicDependency;
use crate::records::multiple_nonviable_overloads::MultipleNonviableOverloads;
use crate::records::non_strict_function_definition_error::NonStrictFunctionDefinitionError;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::not_a_table::NotATable;
use crate::records::occurs_check_failed::OccursCheckFailed;
use crate::records::only_tables_can_have_methods::OnlyTablesCanHaveMethods;
use crate::records::optional_value_access::OptionalValueAccess;
use crate::records::pack_where_clause_needed::PackWhereClauseNeeded;
use crate::records::property_access_violation::PropertyAccessViolation;
use crate::records::recursive_restraint_violation::RecursiveRestraintViolation;
use crate::records::reserved_identifier::ReservedIdentifier;
use crate::records::swapped_generic_type_parameter::SwappedGenericTypeParameter;
use crate::records::syntax_error::SyntaxError;
use crate::records::type_instantiation_count_mismatch::TypeInstantiationCountMismatch;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::type_pack_mismatch::TypePackMismatch;
use crate::records::types_are_unrelated::TypesAreUnrelated;
use crate::records::unapplied_type_function::UnappliedTypeFunction;
use crate::records::unexpected_array_like_table_item::UnexpectedArrayLikeTableItem;
use crate::records::unexpected_type_in_subtyping::UnexpectedTypeInSubtyping;
use crate::records::unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping;
use crate::records::unification_too_complex::UnificationTooComplex;
use crate::records::uninhabited_type_function::UninhabitedTypeFunction;
use crate::records::uninhabited_type_pack_function::UninhabitedTypePackFunction;
use crate::records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp;
use crate::records::unknown_property::UnknownProperty;
use crate::records::unknown_require::UnknownRequire;
use crate::records::unknown_symbol::UnknownSymbol;
use crate::records::user_defined_type_function_error::UserDefinedTypeFunctionError;
use crate::records::where_clause_needed::WhereClauseNeeded;

use crate::enums::interesting_edge_case::InterestingEdgeCase;
use crate::enums::reason::Reason;

/// C++ `errorToString<T>` is an `if constexpr` switch over the concrete error
/// type. Rust models the per-type compile-time dispatch with a trait
/// implemented once per error record; the generic function delegates to it.
pub trait ErrorToString {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result;
}

pub fn error_to_string<T: ErrorToString>(stream: &mut dyn Write, err: &T) -> core::fmt::Result {
    err.error_to_string_impl(stream)
}

impl ErrorToString for TypeMismatch {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "TypeMismatch {{ {}, {} }}",
            to_string_type_id(self.wanted_type),
            to_string_type_id(self.given_type)
        )
    }
}

impl ErrorToString for UnknownSymbol {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "UnknownSymbol {{ {} , context {} }}",
            self.name, self.context as i32
        )
    }
}

impl ErrorToString for UnknownProperty {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "UnknownProperty {{ {}, key = {} }}",
            to_string_type_id(self.table),
            self.key
        )
    }
}

impl ErrorToString for NotATable {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "NotATable {{ {} }}", to_string_type_id(self.ty))
    }
}

impl ErrorToString for CannotExtendTable {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "CannotExtendTable {{ {}, context {}, prop \"{}\" }}",
            to_string_type_id(self.table_type),
            self.context as i32,
            self.prop
        )
    }
}

impl ErrorToString for CannotCompareUnrelatedTypes {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "CannotCompareUnrelatedTypes {{ {}, {}, op '{}' }}",
            to_string_type_id(self.left),
            to_string_type_id(self.right),
            to_string_ast_expr_binary_op(self.op)
        )
    }
}

impl ErrorToString for OnlyTablesCanHaveMethods {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "OnlyTablesCanHaveMethods {{ {} }}",
            to_string_type_id(self.table_type)
        )
    }
}

impl ErrorToString for DuplicateTypeDefinition {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "DuplicateTypeDefinition {{ {} }}", self.name)
    }
}

impl ErrorToString for CountMismatch {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "CountMismatch {{ expected {}, got {}, context {} }}",
            self.expected, self.actual, self.context as i32
        )
    }
}

impl ErrorToString for FunctionDoesNotTakeSelf {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "FunctionDoesNotTakeSelf {{ }}")
    }
}

impl ErrorToString for FunctionRequiresSelf {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "FunctionRequiresSelf {{ }}")
    }
}

impl ErrorToString for OccursCheckFailed {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "OccursCheckFailed {{ }}")
    }
}

impl ErrorToString for UnknownRequire {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "UnknownRequire {{ {} }}", self.module_path)
    }
}

impl ErrorToString for IncorrectGenericParameterCount {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "IncorrectGenericParameterCount {{ name = {}",
            self.name
        )?;

        if !self.type_fun.type_params.is_empty() || !self.type_fun.type_pack_params.is_empty() {
            write!(stream, "<")?;
            let mut first = true;
            for param in &self.type_fun.type_params {
                if first {
                    first = false;
                } else {
                    write!(stream, ", ")?;
                }

                write!(stream, "{}", to_string_type_id(param.ty))?;
            }

            for param in &self.type_fun.type_pack_params {
                if first {
                    first = false;
                } else {
                    write!(stream, ", ")?;
                }

                write!(stream, "{}", to_string_type_pack_id(param.tp))?;
            }

            write!(stream, ">")?;
        }

        write!(
            stream,
            ", typeFun = {}, actualCount = {} }}",
            to_string_type_id(self.type_fun.r#type),
            self.actual_parameters
        )
    }
}

impl ErrorToString for SyntaxError {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "SyntaxError {{ {} }}", self.message)
    }
}

impl ErrorToString for CodeTooComplex {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "CodeTooComplex {{}}")
    }
}

impl ErrorToString for UnificationTooComplex {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "UnificationTooComplex {{}}")
    }
}

impl ErrorToString for UnknownPropButFoundLikeProp {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "UnknownPropButFoundLikeProp {{ key = '{}', suggested = {{ ",
            self.key
        )?;

        let mut first = true;
        for name in &self.candidates {
            if first {
                first = false;
            } else {
                write!(stream, ", ")?;
            }

            write!(stream, "'{}'", name)?;
        }

        write!(stream, " }}, table = {} }} ", to_string_type_id(self.table))
    }
}

impl ErrorToString for GenericError {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "GenericError {{ {} }}", self.message)
    }
}

impl ErrorToString for InternalError {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "InternalError {{ {} }}", self.message)
    }
}

impl ErrorToString for ConstraintSolvingIncompleteError {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "ConstraintSolvingIncompleteError {{}}")
    }
}

impl ErrorToString for CannotCallNonFunction {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "CannotCallNonFunction {{ {} }}",
            to_string_type_id(self.ty)
        )
    }
}

impl ErrorToString for ExtraInformation {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "ExtraInformation {{ {} }}", self.message)
    }
}

impl ErrorToString for DeprecatedApiUsed {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "DeprecatedApiUsed {{ {}, useInstead = {} }}",
            self.symbol, self.use_instead
        )
    }
}

impl ErrorToString for ModuleHasCyclicDependency {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "ModuleHasCyclicDependency {{")?;

        let mut first = true;
        for name in &self.cycle {
            if first {
                first = false;
            } else {
                write!(stream, ", ")?;
            }

            write!(stream, "{}", name)?;
        }

        write!(stream, "}}")
    }
}

impl ErrorToString for IllegalRequire {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "IllegalRequire {{ {}, reason = {} }}",
            self.moduleName, self.reason
        )
    }
}

impl ErrorToString for FunctionExitsWithoutReturning {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "FunctionExitsWithoutReturning {{{}}}",
            to_string_type_pack_id(self.expected_return_type)
        )
    }
}

impl ErrorToString for DuplicateGenericParameter {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "DuplicateGenericParameter {{ {} }}",
            self.parameterName
        )
    }
}

impl ErrorToString for CannotInferBinaryOperation {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        let suggested = self.suggested_to_annotate.as_deref().unwrap_or("");
        write!(
            stream,
            "CannotInferBinaryOperation {{ op = {}, suggested = '{}', kind {}}}",
            to_string_ast_expr_binary_op(self.op),
            suggested,
            self.kind as i32
        )
    }
}

impl ErrorToString for MissingProperties {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "MissingProperties {{ superType = '{}', subType = '{}', properties = {{ ",
            to_string_type_id(self.super_type),
            to_string_type_id(self.sub_type)
        )?;

        let mut first = true;
        for name in &self.properties {
            if first {
                first = false;
            } else {
                write!(stream, ", ")?;
            }

            write!(stream, "'{}'", name)?;
        }

        write!(stream, " }}, context {} }} ", self.context as i32)
    }
}

impl ErrorToString for SwappedGenericTypeParameter {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "SwappedGenericTypeParameter {{ name = '{}', kind = {} }}",
            self.name, self.kind as i32
        )
    }
}

impl ErrorToString for OptionalValueAccess {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "OptionalValueAccess {{ optional = '{}' }}",
            to_string_type_id(self.optional)
        )
    }
}

impl ErrorToString for MissingUnionProperty {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "MissingUnionProperty {{ type = '{}', missing = {{ ",
            to_string_type_id(self.r#type)
        )?;

        let mut first = true;
        for ty in &self.missing {
            if first {
                first = false;
            } else {
                write!(stream, ", ")?;
            }

            write!(stream, "'{}'", to_string_type_id(*ty))?;
        }

        write!(stream, " }}, key = '{}' }}", self.key)
    }
}

impl ErrorToString for TypesAreUnrelated {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "TypesAreUnrelated {{ left = '{}', right = '{}' }}",
            to_string_type_id(self.left),
            to_string_type_id(self.right)
        )
    }
}

impl ErrorToString for NormalizationTooComplex {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "NormalizationTooComplex {{ }}")
    }
}

impl ErrorToString for TypePackMismatch {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "TypePackMismatch {{ wanted = '{}', given = '{}' }}",
            to_string_type_pack_id(self.wanted_tp),
            to_string_type_pack_id(self.given_tp)
        )
    }
}

impl ErrorToString for DynamicPropertyLookupOnExternTypesUnsafe {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "DynamicPropertyLookupOnExternTypesUnsafe {{ {} }}",
            to_string_type_id(self.ty)
        )
    }
}

impl ErrorToString for UninhabitedTypeFunction {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "UninhabitedTypeFunction {{ {} }}",
            to_string_type_id(self.ty)
        )
    }
}

impl ErrorToString for ExplicitFunctionAnnotationRecommended {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        let mut rec_args = String::from("[");
        for (s, t) in &self.recommended_args {
            rec_args.push_str(" ");
            rec_args.push_str(s);
            rec_args.push_str(": ");
            rec_args.push_str(&to_string_type_id(*t));
        }
        rec_args.push_str(" ]");
        write!(
            stream,
            "ExplicitFunctionAnnotationRecommended {{ recommendedReturn = '{}', recommendedArgs = {}}}",
            to_string_type_id(self.recommended_return),
            rec_args
        )
    }
}

impl ErrorToString for UninhabitedTypePackFunction {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "UninhabitedTypePackFunction {{ {} }}",
            to_string_type_pack_id(self.tp)
        )
    }
}

impl ErrorToString for WhereClauseNeeded {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "WhereClauseNeeded {{ {} }}",
            to_string_type_id(self.ty)
        )
    }
}

impl ErrorToString for PackWhereClauseNeeded {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "PackWhereClauseNeeded {{ {} }}",
            to_string_type_pack_id(self.tp)
        )
    }
}

impl ErrorToString for CheckedFunctionCallError {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "CheckedFunctionCallError {{ expected = '{}', passed = '{}', checkedFunctionName = {}, argumentIndex = {} }}",
            to_string_type_id(self.expected),
            to_string_type_id(self.passed),
            self.checkedFunctionName,
            self.argumentIndex
        )
    }
}

impl ErrorToString for NonStrictFunctionDefinitionError {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "NonStrictFunctionDefinitionError {{ functionName = '{}', argument = '{}', argumentType = '{}' }}",
            self.function_name,
            self.argument,
            to_string_type_id(self.argument_type)
        )
    }
}

impl ErrorToString for PropertyAccessViolation {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "PropertyAccessViolation {{ table = {}, prop = '{}', context = {} }}",
            to_string_type_id(self.table),
            self.key,
            self.context as i32
        )
    }
}

impl ErrorToString for CheckedFunctionIncorrectArgs {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "CheckedFunction {{  functionName = '{}, expected = {}, actual = {}}}",
            self.functionName, self.expected, self.actual
        )
    }
}

impl ErrorToString for UnexpectedTypeInSubtyping {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "UnexpectedTypeInSubtyping {{  ty = '{}' }}",
            to_string_type_id(self.ty)
        )
    }
}

impl ErrorToString for UnexpectedTypePackInSubtyping {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "UnexpectedTypePackInSubtyping {{  tp = '{}' }}",
            to_string_type_pack_id(self.tp)
        )
    }
}

impl ErrorToString for UserDefinedTypeFunctionError {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "UserDefinedTypeFunctionError {{ {} }}",
            self.message
        )
    }
}

impl ErrorToString for BuiltInTypeFunctionError {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "BuiltInTypeFunctionError {{ {} }}",
            to_string_type_function_error(&self.error)
        )
    }
}

impl ErrorToString for ReservedIdentifier {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "ReservedIdentifier {{ {} }}", self.name)
    }
}

impl ErrorToString for CannotAssignToNever {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "CannotAssignToNever {{ rvalueType = '{}', reason = '",
            to_string_type_id(self.rhsType)
        )?;

        // C++: stream << err.reason; (operator<< for CannotAssignToNever::Reason)
        match self.reason {
            Reason::PropertyNarrowed => write!(stream, "PropertyNarrowed")?,
            #[allow(unreachable_patterns)]
            _ => write!(stream, "UnknownReason")?,
        }

        write!(stream, "', cause = {{ ")?;

        let mut first = true;
        for ty in &self.cause {
            if first {
                first = false;
            } else {
                write!(stream, ", ")?;
            }

            write!(stream, "'{}'", to_string_type_id(*ty))?;
        }

        write!(stream, " }} }} ")
    }
}

impl ErrorToString for UnexpectedArrayLikeTableItem {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "UnexpectedArrayLikeTableItem {{}}")
    }
}

impl ErrorToString for CannotCheckDynamicStringFormatCalls {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "CannotCheckDynamicStringFormatCalls {{}}")
    }
}

impl ErrorToString for GenericTypeCountMismatch {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "GenericTypeCountMismatch {{ subTyGenericCount = {}, superTyGenericCount = {} }}",
            self.sub_ty_generic_count, self.super_ty_generic_count
        )
    }
}

impl ErrorToString for GenericTypePackCountMismatch {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "GenericTypePackCountMismatch {{ subTyGenericPackCount = {}, superTyGenericPackCount = {} }}",
            self.subTyGenericPackCount, self.superTyGenericPackCount
        )
    }
}

impl ErrorToString for MultipleNonviableOverloads {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "MultipleNonviableOverloads {{ attemptedArgCount = {} }}",
            self.attempted_arg_count
        )
    }
}

impl ErrorToString for RecursiveRestraintViolation {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "RecursiveRestraintViolation")
    }
}

impl ErrorToString for GenericBoundsMismatch {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "GenericBoundsMismatch {{ genericName = {}, lowerBounds = [",
            self.generic_name
        )?;
        for i in 0..self.lower_bounds.len() {
            if i > 0 {
                write!(stream, ", ")?;
            }
            write!(stream, "{}", to_string_type_id(self.lower_bounds[i]))?;
        }
        write!(stream, "], upperBounds = [")?;
        for i in 0..self.upper_bounds.len() {
            if i > 0 {
                write!(stream, ", ")?;
            }
            write!(stream, "{}", to_string_type_id(self.upper_bounds[i]))?;
        }
        write!(stream, "] }}")
    }
}

impl ErrorToString for InstantiateGenericsOnNonFunction {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "InstantiateGenericsOnNonFunctionInstantiateGenericsOnNonFunction {{ interestingEdgeCase = "
        )?;

        // C++: stream << err.interestingEdgeCase; (operator<< for InterestingEdgeCase)
        match self.interesting_edge_case {
            InterestingEdgeCase::None => write!(stream, "None")?,
            InterestingEdgeCase::MetatableCall => write!(stream, "MetatableCall")?,
            InterestingEdgeCase::Intersection => write!(stream, "Intersection")?,
            #[allow(unreachable_patterns)]
            _ => {
                luaur_common::LUAU_ASSERT!(false);
                write!(stream, "Unknown")?;
            }
        }

        write!(stream, " }}")
    }
}

impl ErrorToString for TypeInstantiationCountMismatch {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        let function_name = self.functionName.as_deref().unwrap_or("<unknown>");
        write!(
            stream,
            "TypeInstantiationCountMismatch {{ functionName = {}, functionType = {}, providedTypes = {}, maximumTypes = {}, providedTypePacks = {}, maximumTypePacks = {} }}",
            function_name,
            to_string_type_id(self.functionType),
            self.providedTypes,
            self.maximumTypes,
            self.providedTypePacks,
            self.maximumTypePacks
        )
    }
}

impl ErrorToString for UnappliedTypeFunction {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(stream, "UnappliedTypeFunction {{}}")
    }
}

impl ErrorToString for AmbiguousFunctionCall {
    fn error_to_string_impl(&self, stream: &mut dyn Write) -> core::fmt::Result {
        write!(
            stream,
            "AmbiguousFunctionCall {{ {}, {} }}",
            to_string_type_id(self.function),
            to_string_type_pack_id(self.arguments)
        )
    }
}
