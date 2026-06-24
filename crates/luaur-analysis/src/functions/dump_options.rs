use crate::records::to_string_options::ToStringOptions;

pub fn dump_options() -> &'static mut ToStringOptions {
    static mut OPTIONS: Option<ToStringOptions> = None;

    unsafe {
        if OPTIONS.is_none() {
            let mut opts = ToStringOptions::to_string_options(true);
            opts.exhaustive = true;
            opts.function_type_arguments = true;
            opts.max_table_length = 0;
            opts.max_type_length = 0;
            OPTIONS = Some(opts);
        }
        OPTIONS.as_mut().unwrap_unchecked()
    }
}
