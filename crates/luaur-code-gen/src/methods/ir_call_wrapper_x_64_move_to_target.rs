use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::call_argument::CallArgument;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;

impl IrCallWrapperX64 {
    pub fn move_to_target(&mut self, arg: &mut CallArgument) {
        let source_cat = arg.source.cat;
        if source_cat == CategoryX64::reg {
            let source = arg.source.base;

            if source.size() == SizeX64::xmmword {
                unsafe {
                    (*self.build).vmovsd_operand_x_64_operand_x_64_operand_x_64(
                        arg.target,
                        source.into(),
                        source.into(),
                    )
                };
            } else {
                unsafe { (*self.build).mov(arg.target, source.into()) };
            }
        } else if source_cat == CategoryX64::imm {
            unsafe { (*self.build).mov(arg.target, arg.source) };
        } else {
            if arg.source.memSize == SizeX64::none {
                unsafe {
                    (*self.build).lea_operand_x_64_operand_x_64(arg.target, arg.source);
                };
            } else if arg.target.base.size() == SizeX64::xmmword
                && arg.source.memSize == SizeX64::xmmword
            {
                unsafe { (*self.build).vmovups(arg.target, arg.source) };
            } else if arg.target.base.size() == SizeX64::xmmword {
                unsafe {
                    (*self.build).vmovsd_operand_x_64_operand_x_64(arg.target, arg.source);
                };
            } else {
                unsafe { (*self.build).mov(arg.target, arg.source) };
            }
        }
    }
}
