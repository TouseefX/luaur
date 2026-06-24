use crate::functions::default_luau_print_line::default_luau_print_line;
use crate::type_aliases::print_line_proc_type_infer::PrintLineProc;

extern "C" fn default_luau_print_line_wrapper(s: &String) {
    default_luau_print_line(s)
}

pub fn reset_print_line() {
    unsafe {
        crate::functions::set_print_line::luauPrintLine = Some(default_luau_print_line_wrapper);
    }
}
