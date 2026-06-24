use crate::records::cli_file_resolver::CliFileResolver;
use alloc::string::String;
use alloc::string::ToString;

#[allow(non_snake_case)]
pub unsafe fn cli_file_resolver_get_human_readable_module_name(
    _this: *const CliFileResolver,
    name: &alloc::string::String,
) -> String {
    if name == "-" {
        "stdin".to_string()
    } else {
        name.clone()
    }
}
