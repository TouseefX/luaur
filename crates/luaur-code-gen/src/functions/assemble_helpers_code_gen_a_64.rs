use crate::functions::emit_clear_native_flag_code_gen_a_64::emit_clear_native_flag_assembly_builder_a_64;
use crate::functions::emit_continue_call::emitContinueCall;
use crate::functions::emit_exit_code_gen_a_64::emit_exit_assembly_builder_a_64_bool;
use crate::functions::emit_interrupt_code_gen_a_64::emit_interrupt;
use crate::functions::emit_return_code_gen_a_64::emit_return_assembly_builder_a_64_module_helpers;
use crate::functions::emit_update_pc_for_exit_code_gen_a_64::emit_update_pc_for_exit_assembly_builder_a_64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::module_helpers::ModuleHelpers;

pub fn assemble_helpers(build: &mut AssemblyBuilderA64, helpers: &mut ModuleHelpers) {
    if build.log_text {
        build.log_append(format_args!("; updatePcAndContinueInVm\n"));
    }
    build.set_label_label(&mut helpers.updatePcAndContinueInVm);
    emit_update_pc_for_exit_assembly_builder_a_64(build);

    if build.log_text {
        build.log_append(format_args!("; exitContinueVmClearNativeFlag\n"));
    }
    build.set_label_label(&mut helpers.exitContinueVmClearNativeFlag);
    emit_clear_native_flag_assembly_builder_a_64(build);

    if build.log_text {
        build.log_append(format_args!("; exitContinueVm\n"));
    }
    build.set_label_label(&mut helpers.exitContinueVm);
    emit_exit_assembly_builder_a_64_bool(build, true);

    if build.log_text {
        build.log_append(format_args!("; exitNoContinueVm\n"));
    }
    build.set_label_label(&mut helpers.exitNoContinueVm);
    emit_exit_assembly_builder_a_64_bool(build, false);

    if build.log_text {
        build.log_append(format_args!("; interrupt\n"));
    }
    build.set_label_label(&mut helpers.interrupt);
    emit_interrupt(build);

    if build.log_text {
        build.log_append(format_args!("; return\n"));
    }
    build.set_label_label(&mut helpers.return_);
    emit_return_assembly_builder_a_64_module_helpers(build, helpers);

    if build.log_text {
        build.log_append(format_args!("; continueCall\n"));
    }
    build.set_label_label(&mut helpers.continueCall);
    emitContinueCall(build, helpers);
}
