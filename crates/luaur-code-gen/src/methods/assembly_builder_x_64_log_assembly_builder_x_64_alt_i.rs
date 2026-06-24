use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn log_operand_x_64(&mut self, op: OperandX64) {
        match op.cat {
            CategoryX64::reg => {
                let reg_name = self.get_register_name(op.base);
                self.log_append(format_args!("{}", reg_name));
            }
            CategoryX64::mem => {
                if op.base == RegisterX64::rip {
                    if op.memSize != SizeX64::none {
                        self.log_append(format_args!("{} ptr ", self.get_size_name(op.memSize)));
                    }
                    // C++ `logAppend("[.start%+d]", op.imm)` — `%+d` always shows a sign.
                    self.log_append(format_args!("[.start{:+}]", op.imm));
                    return;
                }

                if op.memSize != SizeX64::none {
                    self.log_append(format_args!("{} ptr ", self.get_size_name(op.memSize)));
                }

                self.log_append(format_args!("["));

                if op.base != RegisterX64::noreg {
                    let reg_name = self.get_register_name(op.base);
                    self.log_append(format_args!("{}", reg_name));
                }

                if op.index != RegisterX64::noreg {
                    let index_name = self.get_register_name(op.index);
                    self.log_append(format_args!(
                        "{}{}",
                        if op.base != RegisterX64::noreg {
                            "+"
                        } else {
                            ""
                        },
                        index_name
                    ));
                }

                if op.scale != 1 {
                    self.log_append(format_args!("*{}", op.scale));
                }

                if op.imm != 0 {
                    if op.imm >= 0 && op.imm <= 9 {
                        self.log_append(format_args!("+{}", op.imm));
                    } else if op.imm > 0 {
                        self.log_append(format_args!("+0{:X}h", op.imm));
                    } else {
                        self.log_append(format_args!("-0{:X}h", -op.imm));
                    }
                }

                self.text.push_str("]");
            }
            CategoryX64::imm => {
                if op.imm >= 0 && op.imm <= 9 {
                    self.log_append(format_args!("{}", op.imm));
                } else {
                    self.log_append(format_args!("{:X}h", op.imm));
                }
            }
        }
    }
}
