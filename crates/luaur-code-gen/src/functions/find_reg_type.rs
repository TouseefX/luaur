use crate::records::bytecode_reg_type_info::BytecodeRegTypeInfo;
use crate::records::bytecode_type_info::BytecodeTypeInfo;

pub(crate) fn find_reg_type(
    info: &mut BytecodeTypeInfo,
    reg: u8,
    pc: i32,
) -> Option<&mut BytecodeRegTypeInfo> {
    let b_idx = info.reg_type_offsets[reg as usize] as usize;
    let e_idx = info.reg_type_offsets[reg as usize + 1] as usize;

    // Doesn't have info
    if b_idx == e_idx {
        return None;
    }

    // No info after the last live range
    if pc >= info.reg_types[e_idx - 1].endpc {
        return None;
    }

    for i in b_idx..e_idx {
        luaur_common::LUAU_ASSERT!(info.reg_types[i].reg == reg);

        if pc >= info.reg_types[i].startpc && pc < info.reg_types[i].endpc {
            return Some(&mut info.reg_types[i]);
        }
    }

    None
}
