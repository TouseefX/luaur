use crate::functions::create_native_proto_exec_data_native_proto_exec_data::create_native_proto_exec_data_u32_u32;
use crate::functions::get_native_proto_exec_data_header_native_proto_exec_data::get_native_proto_exec_data_header_mut;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::native_proto_exec_data_ptr::NativeProtoExecDataPtr;
use luaur_vm::records::proto::Proto;

pub fn create_native_proto_exec_data(proto: *mut Proto, ir: &IrBuilder) -> NativeProtoExecDataPtr {
    unsafe {
        let extra_data_count = ir.function.extra_native_data.len() as u32;
        let native_exec_data =
            create_native_proto_exec_data_u32_u32((*proto).sizecode as u32, extra_data_count);

        let inst_target = ir.function.entry_location;
        let unassigned_offset = ir.function.end_location - inst_target;
        let data = native_exec_data.as_ptr();

        for i in 0..(*proto).sizecode as usize {
            let bc_mapping = ir.function.bc_mapping[i];

            CODEGEN_ASSERT!(bc_mapping.asm_location >= inst_target);

            *data.add(i) = if bc_mapping.asm_location != !0u32 {
                bc_mapping.asm_location - inst_target
            } else {
                unassigned_offset
            };
        }

        for i in 0..extra_data_count as usize {
            *data.add((*proto).sizecode as usize + i) = ir.function.extra_native_data[i];
        }

        if (*proto).sizecode > 0 {
            *data = 0;
        }

        let header = get_native_proto_exec_data_header_mut(data);
        (*header).entry_offset_or_address = inst_target as usize as *const u8;
        (*header).bytecode_id = (*proto).bytecodeid as u32;
        (*header).bytecode_instruction_count = (*proto).sizecode as u32;
        (*header).extra_data_count = extra_data_count;

        native_exec_data
    }
}
