#[cfg(test)]
#[test]
fn assembly_builder_x_64_alignment_overflow() {
    // Test that alignment correctly resizes the code buffer
    {
        let mut build = luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        build.ret();
        build.align(
            8192,
            luaur_code_gen::enums::alignment_data_x_64::AlignmentDataX64::Nop,
        );
        build.finalize();
    }

    {
        let mut build = luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        build.ret();
        build.align(
            8192,
            luaur_code_gen::enums::alignment_data_x_64::AlignmentDataX64::Int3,
        );
        build.finalize();
    }

    {
        let mut build = luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        for _ in 0..8192 {
            build.int3();
        }
        build.finalize();
    }

    {
        let mut build = luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        build.ret();
        build.align(
            8192,
            luaur_code_gen::enums::alignment_data_x_64::AlignmentDataX64::Ud2,
        );
        build.finalize();
    }
}
