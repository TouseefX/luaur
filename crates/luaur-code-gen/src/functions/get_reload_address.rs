use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::kind_a_64::KindA64;
use crate::functions::get_reload_offset::get_reload_offset;
use crate::functions::vm_const_op::vm_const_op;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::address_a_64::AddressA64;
use crate::records::register_a_64::RegisterA64;
use crate::records::value_restore_location::ValueRestoreLocation;
use luaur_vm::type_aliases::t_value::TValue;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << RegisterA64::INDEX_SHIFT),
    }
}

const R_BASE: RegisterA64 = reg(KindA64::x, 25);
const R_CONSTANTS: RegisterA64 = reg(KindA64::x, 22);
const XZR: RegisterA64 = reg(KindA64::x, 31);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

pub fn get_reload_address(location: ValueRestoreLocation) -> AddressA64 {
    let op = location.op;

    if op.kind() == IrOpKind::VmReg {
        let offset = vm_reg_op(op) as i32 * core::mem::size_of::<TValue>() as i32
            + get_reload_offset(location.kind);
        return mem(R_BASE, offset);
    }

    // loads are 4/8/16 bytes; we conservatively limit the offset to fit assuming a 4b index
    if op.kind() == IrOpKind::VmConst
        && (vm_const_op(op) as usize * core::mem::size_of::<TValue>() <= AddressA64::kMaxOffset * 4)
    {
        let offset = vm_const_op(op) as i32 * core::mem::size_of::<TValue>() as i32
            + get_reload_offset(location.kind);
        return mem(R_CONSTANTS, offset);
    }

    AddressA64::address_a_64_register_a_64_register_a_64(XZR, XZR)
}
