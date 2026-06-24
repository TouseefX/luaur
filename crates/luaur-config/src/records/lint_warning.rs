use crate::enums::code::Code;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LintWarning {
    pub code: Code,
    pub location: Location,
    pub text: alloc::string::String,
}

impl LintWarning {
    pub const Code_Unknown: Code = Code::Code_Unknown;
    pub const Code_UnknownGlobal: Code = Code::Code_UnknownGlobal;
    pub const Code_DeprecatedGlobal: Code = Code::Code_DeprecatedGlobal;
    pub const Code_GlobalUsedAsLocal: Code = Code::Code_GlobalUsedAsLocal;
    pub const Code_LocalShadow: Code = Code::Code_LocalShadow;
    pub const Code_SameLineStatement: Code = Code::Code_SameLineStatement;
    pub const Code_MultiLineStatement: Code = Code::Code_MultiLineStatement;
    pub const Code_LocalUnused: Code = Code::Code_LocalUnused;
    pub const Code_FunctionUnused: Code = Code::Code_FunctionUnused;
    pub const Code_ImportUnused: Code = Code::Code_ImportUnused;
    pub const Code_BuiltinGlobalWrite: Code = Code::Code_BuiltinGlobalWrite;
    pub const Code_PlaceholderRead: Code = Code::Code_PlaceholderRead;
    pub const Code_UnreachableCode: Code = Code::Code_UnreachableCode;
    pub const Code_UnknownType: Code = Code::Code_UnknownType;
    pub const Code_ForRange: Code = Code::Code_ForRange;
    pub const Code_UnbalancedAssignment: Code = Code::Code_UnbalancedAssignment;
    pub const Code_ImplicitReturn: Code = Code::Code_ImplicitReturn;
    pub const Code_DuplicateLocal: Code = Code::Code_DuplicateLocal;
    pub const Code_FormatString: Code = Code::Code_FormatString;
    pub const Code_TableLiteral: Code = Code::Code_TableLiteral;
    pub const Code_UninitializedLocal: Code = Code::Code_UninitializedLocal;
    pub const Code_DuplicateFunction: Code = Code::Code_DuplicateFunction;
    pub const Code_DeprecatedApi: Code = Code::Code_DeprecatedApi;
    pub const Code_TableOperations: Code = Code::Code_TableOperations;
    pub const Code_DuplicateCondition: Code = Code::Code_DuplicateCondition;
    pub const Code_MisleadingAndOr: Code = Code::Code_MisleadingAndOr;
    pub const Code_CommentDirective: Code = Code::Code_CommentDirective;
    pub const Code_IntegerParsing: Code = Code::Code_IntegerParsing;
    pub const Code_ComparisonPrecedence: Code = Code::Code_ComparisonPrecedence;
    pub const Code_RedundantNativeAttribute: Code = Code::Code_RedundantNativeAttribute;
    pub const Code__Count: Code = Code::Code__Count;
}
