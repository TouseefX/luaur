use crate::enums::code::Code;
use crate::records::lint_warning::LintWarning;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl LintWarning {
    pub fn get_name(code: Code) -> &'static str {
        LUAU_ASSERT!((code as i32) < (Code::Code__Count as i32));

        const K_WARNING_NAMES: &[&str] = &[
            "Unknown",
            "UnknownGlobal",
            "DeprecatedGlobal",
            "GlobalUsedAsLocal",
            "LocalShadow",
            "SameLineStatement",
            "MultiLineStatement",
            "LocalUnused",
            "FunctionUnused",
            "ImportUnused",
            "BuiltinGlobalWrite",
            "PlaceholderRead",
            "UnreachableCode",
            "UnknownType",
            "ForRange",
            "UnbalancedAssignment",
            "ImplicitReturn",
            "DuplicateLocal",
            "FormatString",
            "TableLiteral",
            "UninitializedLocal",
            "DuplicateFunction",
            "DeprecatedApi",
            "TableOperations",
            "DuplicateCondition",
            "MisleadingAndOr",
            "CommentDirective",
            "IntegerParsing",
            "ComparisonPrecedence",
            "RedundantNativeAttribute",
        ];

        K_WARNING_NAMES[code as usize]
    }
}
