use crate::enums::code::Code;
use crate::functions::parse_lint_rule_string_for_code::parse_lint_rule_string_for_code;
use crate::records::lint_options::LintOptions;
use crate::records::lint_warning::LintWarning;
use crate::type_aliases::error::Error;

pub fn parse_lint_rule_string(
    enabled_lints: &mut LintOptions,
    fatal_lints: &mut LintOptions,
    warning_name: &str,
    value: &str,
    compat: bool,
) -> Error {
    if warning_name == "*" {
        let mut code = LintWarning::Code_Unknown;
        while code != LintWarning::Code__Count {
            if let Some(err) =
                parse_lint_rule_string_for_code(enabled_lints, fatal_lints, code, value, compat)
            {
                return Some(alloc::format!("In key {}: {}", warning_name, err));
            }

            code = match code {
                Code::Code_Unknown => LintWarning::Code_UnknownGlobal,
                Code::Code_UnknownGlobal => LintWarning::Code_DeprecatedGlobal,
                Code::Code_DeprecatedGlobal => LintWarning::Code_GlobalUsedAsLocal,
                Code::Code_GlobalUsedAsLocal => LintWarning::Code_LocalShadow,
                Code::Code_LocalShadow => LintWarning::Code_SameLineStatement,
                Code::Code_SameLineStatement => LintWarning::Code_MultiLineStatement,
                Code::Code_MultiLineStatement => LintWarning::Code_LocalUnused,
                Code::Code_LocalUnused => LintWarning::Code_FunctionUnused,
                Code::Code_FunctionUnused => LintWarning::Code_ImportUnused,
                Code::Code_ImportUnused => LintWarning::Code_BuiltinGlobalWrite,
                Code::Code_BuiltinGlobalWrite => LintWarning::Code_PlaceholderRead,
                Code::Code_PlaceholderRead => LintWarning::Code_UnreachableCode,
                Code::Code_UnreachableCode => LintWarning::Code_UnknownType,
                Code::Code_UnknownType => LintWarning::Code_ForRange,
                Code::Code_ForRange => LintWarning::Code_UnbalancedAssignment,
                Code::Code_UnbalancedAssignment => LintWarning::Code_ImplicitReturn,
                Code::Code_ImplicitReturn => LintWarning::Code_DuplicateLocal,
                Code::Code_DuplicateLocal => LintWarning::Code_FormatString,
                Code::Code_FormatString => LintWarning::Code_TableLiteral,
                Code::Code_TableLiteral => LintWarning::Code_UninitializedLocal,
                Code::Code_UninitializedLocal => LintWarning::Code_DuplicateFunction,
                Code::Code_DuplicateFunction => LintWarning::Code_DeprecatedApi,
                Code::Code_DeprecatedApi => LintWarning::Code_TableOperations,
                Code::Code_TableOperations => LintWarning::Code_DuplicateCondition,
                Code::Code_DuplicateCondition => LintWarning::Code_MisleadingAndOr,
                Code::Code_MisleadingAndOr => LintWarning::Code_CommentDirective,
                Code::Code_CommentDirective => LintWarning::Code_IntegerParsing,
                Code::Code_IntegerParsing => LintWarning::Code_ComparisonPrecedence,
                Code::Code_ComparisonPrecedence => LintWarning::Code_RedundantNativeAttribute,
                Code::Code_RedundantNativeAttribute => LintWarning::Code__Count,
                // If new codes are added later, fall back to the sentinel to avoid infinite loops.
                _ => LintWarning::Code__Count,
            };
        }
    } else {
        let code = LintWarning::parse_name(warning_name);

        if code == Code::Code_Unknown {
            return Some(alloc::format!("Unknown lint {}", warning_name));
        }

        if let Some(err) =
            parse_lint_rule_string_for_code(enabled_lints, fatal_lints, code, value, compat)
        {
            return Some(alloc::format!("In key {}: {}", warning_name, err));
        }
    }

    None
}
