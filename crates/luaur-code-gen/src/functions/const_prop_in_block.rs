use crate::enums::ir_block_kind::IrBlockKind;
use crate::functions::apply_substitutions_ir_utils_alt_b::apply_substitutions_ir_function_ir_inst;
use crate::functions::const_prop_in_inst::const_prop_in_inst;
use crate::functions::fold_constants::fold_constants;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;

const K_BLOCK_FLAG_SAFE_ENV_CHECK: u8 = 1 << 0;

pub fn const_prop_in_block(build: &mut IrBuilder, block: &mut IrBlock, state: &mut ConstPropState) {
    let function: *mut crate::records::ir_function::IrFunction = &mut build.function;

    if (block.flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
        state.in_safe_env = true;
    }

    for index in block.start..=block.finish {
        unsafe {
            let inst = &mut (&mut (*function).instructions)[index as usize] as *mut _;

            apply_substitutions_ir_function_ir_inst(&mut *function, &mut *inst);
            fold_constants(build, &mut *function, block, index);
            const_prop_in_inst(state, build, &mut *function, block, &mut *inst, index);
        }

        if block.kind == IrBlockKind::Dead {
            break;
        }
    }
}
