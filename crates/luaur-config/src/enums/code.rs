#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Code {
    Code_Unknown = 0,
    Code_UnknownGlobal = 1,
    Code_DeprecatedGlobal = 2,
    Code_GlobalUsedAsLocal = 3,
    Code_LocalShadow = 4,
    Code_SameLineStatement = 5,
    Code_MultiLineStatement = 6,
    Code_LocalUnused = 7,
    Code_FunctionUnused = 8,
    Code_ImportUnused = 9,
    Code_BuiltinGlobalWrite = 10,
    Code_PlaceholderRead = 11,
    Code_UnreachableCode = 12,
    Code_UnknownType = 13,
    Code_ForRange = 14,
    Code_UnbalancedAssignment = 15,
    Code_ImplicitReturn = 16,
    Code_DuplicateLocal = 17,
    Code_FormatString = 18,
    Code_TableLiteral = 19,
    Code_UninitializedLocal = 20,
    Code_DuplicateFunction = 21,
    Code_DeprecatedApi = 22,
    Code_TableOperations = 23,
    Code_DuplicateCondition = 24,
    Code_MisleadingAndOr = 25,
    Code_CommentDirective = 26,
    Code_IntegerParsing = 27,
    Code_ComparisonPrecedence = 28,
    Code_RedundantNativeAttribute = 29,
    Code__Count = 30,
}

impl Default for Code {
    fn default() -> Self {
        Code::Code_Unknown
    }
}
