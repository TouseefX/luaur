extern crate alloc;

use crate::functions::to_dot::to_dot;
use crate::records::ir_function::IrFunction;
use alloc::string::String;

pub fn dump_dot(function: &IrFunction, include_inst: bool) -> String {
    let result = to_dot(function, include_inst);

    std::println!("{}", result);

    result
}
