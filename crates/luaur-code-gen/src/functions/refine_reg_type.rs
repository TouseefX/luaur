use crate::functions::find_reg_type::find_reg_type;
use crate::records::bytecode_reg_type_info::BytecodeRegTypeInfo;
use crate::records::bytecode_reg_type_info::LBC_TYPE_ANY;
use crate::records::bytecode_type_info::BytecodeTypeInfo;

pub(crate) fn refine_reg_type(info: &mut BytecodeTypeInfo, reg: u8, pc: i32, ty: u8) {
    if ty != LBC_TYPE_ANY {
        if let Some(reg_type) = find_reg_type(info, reg, pc) {
            if reg_type.r#type == LBC_TYPE_ANY {
                reg_type.r#type = ty;
            }
        } else if (reg as usize) < info.argument_types.len() {
            if info.argument_types[reg as usize] == LBC_TYPE_ANY {
                info.argument_types[reg as usize] = ty;
            }
        }
    }
}
