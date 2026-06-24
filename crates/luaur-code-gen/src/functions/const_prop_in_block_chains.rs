use crate::enums::ir_block_kind::IrBlockKind;
use crate::functions::const_prop_in_block_chain::const_prop_in_block_chain;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_builder::IrBuilder;

pub fn const_prop_in_block_chains(build: &mut IrBuilder) {
    let function: *mut crate::records::ir_function::IrFunction = &mut build.function;
    let mut state =
        unsafe { ConstPropState::const_prop_state_const_prop_state(build, &mut *function) };
    let mut visited = unsafe { vec![0u8; (&(*function).blocks).len()] };

    unsafe {
        (&mut (*function).block_exit_tags).resize((&(*function).blocks).len(), Vec::new());

        let block_count = (&(*function).blocks).len();
        for i in 0..block_count {
            let block: *mut crate::records::ir_block::IrBlock = &mut (&mut (*function).blocks)[i];

            if (*block).kind == IrBlockKind::Fallback || (*block).kind == IrBlockKind::Dead {
                continue;
            }

            if visited[i] != 0 {
                continue;
            }

            const_prop_in_block_chain(build, &mut visited, block, &mut state);
        }
    }
}
