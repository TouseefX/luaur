//! C++ free function `lint` (`Analysis/src/Linter.cpp:3517`).

use alloc::vec::Vec;

use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::hot_comment::HotComment;
use luaur_config::enums::code::Code;
use luaur_config::records::lint_options::LintOptions;
use luaur_config::records::lint_warning::LintWarning;

use crate::functions::fill_builtin_globals::fill_builtin_globals;
use crate::functions::has_native_comment_directive::has_native_comment_directive;
use crate::functions::lint_comments::lint_comments;
use crate::records::lint_context::LintContext;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use crate::records::lint_duplicate_condition::LintDuplicateCondition;
use crate::records::lint_duplicate_function::LintDuplicateFunction;
use crate::records::lint_duplicate_local::LintDuplicateLocal;
use crate::records::lint_for_range::LintForRange;
use crate::records::lint_format_string::LintFormatString;
use crate::records::lint_global_local::LintGlobalLocal;
use crate::records::lint_integer_parsing::LintIntegerParsing;
use crate::records::lint_misleading_and_or::LintMisleadingAndOr;
use crate::records::lint_multi_line_statement::LintMultiLineStatement;
use crate::records::lint_same_line_statement::LintSameLineStatement;
use crate::records::lint_table_literal::LintTableLiteral;
use crate::records::lint_table_operations::LintTableOperations;
use crate::records::lint_uninitialized_local::LintUninitializedLocal;
use crate::records::lint_unreachable_code::LintUnreachableCode;
use crate::records::lint_unused_function::LintUnusedFunction;
use crate::records::warning_comparator::WarningComparator;
use crate::type_aliases::scope_ptr_type::ScopePtr;

use crate::methods::lint_implicit_return_process::lint_implicit_return_process;
use crate::methods::lint_local_hygiene_process::lint_local_hygiene_process;
use crate::methods::lint_redundant_native_attribute_process::lint_redundant_native_attribute_process;
use crate::methods::lint_unbalanced_assignment_process::lint_unbalanced_assignment_process;
use crate::methods::lint_unknown_type_process::lint_unknown_type_process;
use crate::records::lint_comparison_precedence::LintComparisonPrecedence;

use crate::records::module::Module;

pub fn lint(
    root: *mut AstStat,
    names: &AstNameTable,
    env: &ScopePtr,
    module: *const Module,
    hotcomments: &[HotComment],
    options: &LintOptions,
) -> Vec<LintWarning> {
    let mut context = LintContext {
        result: Vec::new(),
        options: *options,
        root,
        placeholder: names.get(c"_".as_ptr()),
        builtin_globals: luaur_common::records::dense_hash_map::DenseHashMap::new(
            luaur_ast::records::ast_name::AstName::new(),
        ),
        scope: env.clone(),
        module,
    };

    fill_builtin_globals(&mut context, names, env);

    if context.warning_enabled(Code::Code_UnknownGlobal)
        || context.warning_enabled(Code::Code_DeprecatedGlobal)
        || context.warning_enabled(Code::Code_GlobalUsedAsLocal)
        || context.warning_enabled(Code::Code_PlaceholderRead)
        || context.warning_enabled(Code::Code_BuiltinGlobalWrite)
    {
        LintGlobalLocal::process(&mut context);
    }

    if context.warning_enabled(Code::Code_MultiLineStatement) {
        LintMultiLineStatement::new(core::ptr::null_mut()).process(&mut context);
    }

    if context.warning_enabled(Code::Code_SameLineStatement) {
        LintSameLineStatement::new(core::ptr::null_mut()).process(&mut context);
    }

    if context.warning_enabled(Code::Code_LocalShadow)
        || context.warning_enabled(Code::Code_FunctionUnused)
        || context.warning_enabled(Code::Code_ImportUnused)
        || context.warning_enabled(Code::Code_LocalUnused)
    {
        lint_local_hygiene_process(&mut context);
    }

    if context.warning_enabled(Code::Code_FunctionUnused) {
        LintUnusedFunction::new().process(&mut context);
    }

    if context.warning_enabled(Code::Code_UnreachableCode) {
        LintUnreachableCode::process(&mut context);
    }

    if context.warning_enabled(Code::Code_UnknownType) {
        lint_unknown_type_process(&mut context);
    }

    if context.warning_enabled(Code::Code_ForRange) {
        LintForRange::process(&mut context);
    }

    if context.warning_enabled(Code::Code_UnbalancedAssignment) {
        lint_unbalanced_assignment_process(&mut context);
    }

    if context.warning_enabled(Code::Code_ImplicitReturn) {
        lint_implicit_return_process(&mut context);
    }

    if context.warning_enabled(Code::Code_FormatString) {
        LintFormatString {
            context: core::ptr::null_mut(),
        }
        .process(&mut context);
    }

    if context.warning_enabled(Code::Code_TableLiteral) {
        LintTableLiteral {
            context: core::ptr::null_mut(),
        }
        .process(&mut context);
    }

    if context.warning_enabled(Code::Code_UninitializedLocal) {
        LintUninitializedLocal::process(&mut context);
    }

    if context.warning_enabled(Code::Code_DuplicateFunction) {
        LintDuplicateFunction::new(&mut context as *mut LintContext).process();
    }

    if context.warning_enabled(Code::Code_DeprecatedApi) {
        LintDeprecatedApi {
            context: core::ptr::null_mut(),
            function_type_scope_stack: Vec::new(),
        }
        .process(&mut context);
    }

    if context.warning_enabled(Code::Code_TableOperations) {
        LintTableOperations::process(&mut context);
    }

    if context.warning_enabled(Code::Code_DuplicateCondition) {
        LintDuplicateCondition {
            context: &mut context as *mut LintContext,
        }
        .process();
    }

    if context.warning_enabled(Code::Code_DuplicateLocal) {
        LintDuplicateLocal::process(&mut context);
    }

    if context.warning_enabled(Code::Code_MisleadingAndOr) {
        LintMisleadingAndOr {
            context: core::ptr::null_mut(),
        }
        .process(&mut context);
    }

    if context.warning_enabled(Code::Code_CommentDirective) {
        lint_comments(&mut context, hotcomments);
    }

    if context.warning_enabled(Code::Code_IntegerParsing) {
        LintIntegerParsing::process(&mut context);
    }

    if context.warning_enabled(Code::Code_ComparisonPrecedence) {
        LintComparisonPrecedence::process(&mut context);
    }

    if context.warning_enabled(Code::Code_RedundantNativeAttribute) {
        if has_native_comment_directive(hotcomments) {
            lint_redundant_native_attribute_process(&mut context);
        }
    }

    let comparator = WarningComparator::default();
    context.result.sort_by(|lhs, rhs| {
        let c = comparator.compare_location_location(&lhs.location, &rhs.location);
        if c != 0 {
            return c.cmp(&0);
        }
        (lhs.code as i32).cmp(&(rhs.code as i32))
    });

    context.result
}
