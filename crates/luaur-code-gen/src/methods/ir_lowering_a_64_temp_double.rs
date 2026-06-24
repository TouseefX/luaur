use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::kind_a_64::KindA64;
use crate::functions::get_double_bits::get_double_bits;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;
use crate::records::register_a_64::RegisterA64;

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_temp_double(&mut self, op: IrOp) -> RegisterA64 {
        if op.kind() == IrOpKind::Inst {
            self.ir_lowering_a_64_reg_op(op)
        } else if op.kind() == IrOpKind::Constant {
            let val = self.ir_lowering_a_64_double_op(op);

            if unsafe { (*self.build).is_fmov_supported_fp_64(val) } {
                let temp = self.regs.alloc_temp(KindA64::d);
                unsafe { (*self.build).fmov_register_a_64_f64(temp, val) };
                return temp;
            } else {
                let temp1 = self.regs.alloc_temp(KindA64::x);
                let temp2 = self.regs.alloc_temp(KindA64::d);

                let vali = get_double_bits(val);

                if (vali << 16) == 0 {
                    unsafe {
                        (*self.build).movz(temp1, (vali >> 48) as u16, 48);
                        (*self.build).fmov_register_a_64_register_a_64(temp2, temp1);
                    }
                } else if (vali << 32) == 0 {
                    unsafe {
                        (*self.build).movz(temp1, (vali >> 48) as u16, 48);
                        (*self.build).movk(temp1, (vali >> 32) as u16, 32);
                        (*self.build).fmov_register_a_64_register_a_64(temp2, temp1);
                    }
                } else {
                    unsafe {
                        (*self.build).adr_register_a_64_f64(temp1, val);
                        (*self.build).ldr(
                            temp2,
                            crate::records::address_a_64::AddressA64 {
                                kind: crate::enums::address_kind_a_64::AddressKindA64::imm,
                                base: temp1,
                                offset: RegisterA64::noreg,
                                data: 0,
                            },
                        );
                    }
                }

                return temp2;
            }
        } else {
            CODEGEN_ASSERT!(false, "Unsupported instruction form");
            return RegisterA64::noreg;
        }
    }
}
