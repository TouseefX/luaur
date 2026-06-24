extern crate alloc;

use crate::enums::include_use_info::IncludeUseInfo;
use crate::functions::to_string_ir_dump_alt_g::to_string as to_string_function;
use crate::records::ir_function::IrFunction;
use alloc::string::String;

pub fn dump(function: &mut IrFunction) -> String {
    let result = to_string_function(function, IncludeUseInfo::Yes);

    std::println!("{}", result);

    result
}
