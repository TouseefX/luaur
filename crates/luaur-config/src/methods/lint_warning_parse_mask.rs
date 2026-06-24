use crate::enums::code::Code;
use crate::records::lint_warning::LintWarning;
use luaur_ast::records::hot_comment::HotComment;

impl LintWarning {
    pub fn parse_mask(hotcomments: &[HotComment]) -> u64 {
        let mut result: u64 = 0;

        for hc in hotcomments {
            if !hc.header {
                continue;
            }

            if !hc.content.starts_with("nolint") {
                continue;
            }

            let name_start = hc.content[6..].find(|c: char| c != ' ' && c != '\t');

            match name_start {
                None => return !0u64,
                Some(offset) => {
                    if offset == 0 {
                        continue;
                    }

                    let code = Self::parse_name(&hc.content[6 + offset..]);

                    if code != Code::Code_Unknown {
                        result |= 1u64 << (code as i32);
                    }
                }
            }
        }

        result
    }
}
