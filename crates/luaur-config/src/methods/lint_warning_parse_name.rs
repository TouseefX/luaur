use crate::enums::code::Code;
use crate::records::lint_warning::LintWarning;

impl LintWarning {
    pub fn parse_name(name: &str) -> Code {
        for code_idx in (Code::Code_Unknown as i32)..(Code::Code__Count as i32) {
            // Safety: The loop range is bounded by the enum discriminants.
            let code: Code = unsafe { core::mem::transmute(code_idx) };
            if name == Self::get_name(code) {
                return code;
            }
        }

        Code::Code_Unknown
    }
}
