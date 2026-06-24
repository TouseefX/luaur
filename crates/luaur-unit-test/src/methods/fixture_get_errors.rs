use crate::records::fixture::Fixture;
use alloc::string::String;
use alloc::string::ToString;
use luaur_analysis::records::check_result::CheckResult;

impl Fixture {
    pub fn get_errors(&mut self, cr: &CheckResult) -> String {
        let mut ss = String::new();
        let mut fmt_args = core::format_args!("{}", "");
        self.dump_errors_ostream_vector_type_error(&mut fmt_args, &cr.errors);
        ss
    }
}
