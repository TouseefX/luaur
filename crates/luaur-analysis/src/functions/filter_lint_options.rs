use luaur_ast::enums::mode::Mode;
use luaur_ast::records::hot_comment::HotComment;
use luaur_config::enums::code::Code;
use luaur_config::records::lint_options::LintOptions;
use luaur_config::records::lint_warning::LintWarning;

pub fn filter_lint_options(lint_options: &mut LintOptions, hotcomments: &[HotComment], mode: Mode) {
    let ignore_lints = LintWarning::parse_mask(hotcomments);

    lint_options.warning_mask &= !ignore_lints;

    if mode != Mode::NoCheck {
        lint_options.disable_warning(Code::Code_UnknownGlobal);
    }

    if mode == Mode::Strict {
        lint_options.disable_warning(Code::Code_ImplicitReturn);
    }
}
