use crate::functions::emit_warning::emit_warning;
use crate::functions::fuzzy_match::fuzzy_match;
use crate::records::lint_context::LintContext;
use core::ffi::c_char;
use luaur_ast::records::hot_comment::HotComment;
use luaur_config::enums::code::Code;
use luaur_config::records::lint_warning::LintWarning;

pub fn lint_comments(context: &mut LintContext, hotcomments: &[HotComment]) {
    let mut seen_mode = false;

    for hc in hotcomments {
        // We reserve --!<space> for various informational (non-directive) comments
        if hc.content.is_empty()
            || hc.content.as_bytes().first() == Some(&b' ')
            || hc.content.as_bytes().first() == Some(&b'\t')
        {
            continue;
        }

        if !hc.header {
            emit_warning(
                context,
                Code::Code_CommentDirective,
                hc.location,
                format_args!("Comment directive is ignored because it is placed after the first non-comment token")
            );
        } else {
            let space_pos = hc.content.find(|c| c == ' ' || c == '\t');

            let first = if let Some(pos) = space_pos {
                &hc.content[..pos]
            } else {
                &hc.content[..]
            };

            if first == "nolint" {
                let notspace_pos = if let Some(pos) = space_pos {
                    hc.content[pos..]
                        .find(|c| c != ' ' && c != '\t')
                        .map(|p| pos + p)
                } else {
                    None
                };

                if space_pos.is_none() || notspace_pos.is_none() {
                    // disables all lints
                } else if LintWarning::parse_name(&hc.content[notspace_pos.unwrap()..])
                    == Code::Code_Unknown
                {
                    let rule = &hc.content[notspace_pos.unwrap()..];

                    // skip Unknown
                    let suggestion_ptr =
                        fuzzy_match(rule, K_WARNING_NAMES.as_ptr(), K_WARNING_NAMES.len());
                    if !suggestion_ptr.is_null() {
                        let suggestion =
                            unsafe { core::ffi::CStr::from_ptr(suggestion_ptr).to_string_lossy() };
                        emit_warning(
                            context,
                            Code::Code_CommentDirective,
                            hc.location,
                            format_args!("nolint directive refers to unknown lint rule '{}'; did you mean '{}'?", rule, suggestion)
                        );
                    } else {
                        emit_warning(
                            context,
                            Code::Code_CommentDirective,
                            hc.location,
                            format_args!("nolint directive refers to unknown lint rule '{}'", rule),
                        );
                    }
                }
            } else if first == "nocheck" || first == "nonstrict" || first == "strict" {
                if space_pos.is_some() {
                    emit_warning(
                        context,
                        Code::Code_CommentDirective,
                        hc.location,
                        format_args!("Comment directive with the type checking mode has extra symbols at the end of the line")
                    );
                } else if seen_mode {
                    emit_warning(
                        context,
                        Code::Code_CommentDirective,
                        hc.location,
                        format_args!(
                            "Comment directive with the type checking mode has already been used"
                        ),
                    );
                } else {
                    seen_mode = true;
                }
            } else if first == "optimize" {
                let notspace_pos = if let Some(pos) = space_pos {
                    hc.content[pos..]
                        .find(|c| c != ' ' && c != '\t')
                        .map(|p| pos + p)
                } else {
                    None
                };

                if space_pos.is_none() || notspace_pos.is_none() {
                    emit_warning(
                        context,
                        Code::Code_CommentDirective,
                        hc.location,
                        format_args!("optimize directive requires an optimization level"),
                    );
                } else {
                    let level = &hc.content[notspace_pos.unwrap()..];

                    if level != "0" && level != "1" && level != "2" {
                        emit_warning(
                            context,
                            Code::Code_CommentDirective,
                            hc.location,
                            format_args!("optimize directive uses unknown optimization level '{}', 0..2 expected", level)
                        );
                    }
                }
            } else if first == "native" {
                if space_pos.is_some() {
                    emit_warning(
                        context,
                        Code::Code_CommentDirective,
                        hc.location,
                        format_args!("native directive has extra symbols at the end of the line"),
                    );
                }
            } else {
                const K_HOT_COMMENTS: [*const c_char; 6] = [
                    c"nolint".as_ptr(),
                    c"nocheck".as_ptr(),
                    c"nonstrict".as_ptr(),
                    c"strict".as_ptr(),
                    c"optimize".as_ptr(),
                    c"native".as_ptr(),
                ];

                let suggestion_ptr =
                    fuzzy_match(first, K_HOT_COMMENTS.as_ptr(), K_HOT_COMMENTS.len());
                if !suggestion_ptr.is_null() {
                    let suggestion =
                        unsafe { core::ffi::CStr::from_ptr(suggestion_ptr).to_string_lossy() };
                    emit_warning(
                        context,
                        Code::Code_CommentDirective,
                        hc.location,
                        format_args!(
                            "Unknown comment directive '{}'; did you mean '{}'?",
                            first, suggestion
                        ),
                    );
                } else {
                    emit_warning(
                        context,
                        Code::Code_CommentDirective,
                        hc.location,
                        format_args!("Unknown comment directive '{}'", first),
                    );
                }
            }
        }
    }
}

// kWarningNames array from LinterConfig.h, offset by 1 (skip "Unknown"),
// matching C++ `kWarningNames + 1` with size `Code__Count - 1`.
const K_WARNING_NAMES: [*const c_char; 29] = [
    c"UnknownGlobal".as_ptr(),
    c"DeprecatedGlobal".as_ptr(),
    c"GlobalUsedAsLocal".as_ptr(),
    c"LocalShadow".as_ptr(),
    c"SameLineStatement".as_ptr(),
    c"MultiLineStatement".as_ptr(),
    c"LocalUnused".as_ptr(),
    c"FunctionUnused".as_ptr(),
    c"ImportUnused".as_ptr(),
    c"BuiltinGlobalWrite".as_ptr(),
    c"PlaceholderRead".as_ptr(),
    c"UnreachableCode".as_ptr(),
    c"UnknownType".as_ptr(),
    c"ForRange".as_ptr(),
    c"UnbalancedAssignment".as_ptr(),
    c"ImplicitReturn".as_ptr(),
    c"DuplicateLocal".as_ptr(),
    c"FormatString".as_ptr(),
    c"TableLiteral".as_ptr(),
    c"UninitializedLocal".as_ptr(),
    c"DuplicateFunction".as_ptr(),
    c"DeprecatedApi".as_ptr(),
    c"TableOperations".as_ptr(),
    c"DuplicateCondition".as_ptr(),
    c"MisleadingAndOr".as_ptr(),
    c"CommentDirective".as_ptr(),
    c"IntegerParsing".as_ptr(),
    c"ComparisonPrecedence".as_ptr(),
    c"RedundantNativeAttribute".as_ptr(),
];
