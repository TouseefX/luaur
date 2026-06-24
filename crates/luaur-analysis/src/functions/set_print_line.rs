use crate::type_aliases::print_line_proc_type_infer::PrintLineProc;

#[allow(non_upper_case_globals)]
pub(crate) static mut luauPrintLine: PrintLineProc = None;

#[allow(non_snake_case)]
pub fn setPrintLine(pl: PrintLineProc) {
    unsafe {
        luauPrintLine = pl;
    }
}
