#[cfg(test)]
#[test]
fn assembly_builder_x_64_base_binary_instruction_forms() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{byte, dword, qword, OperandX64};
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    // `size[addr]` — the `[]` stamps the size prefix onto the address expression.
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
        prefix.operator_bracket(addr.into())
    }

    // reg, reg
    check(|b| b.add(R::rax.into(), R::rcx.into()), &[0x48, 0x03, 0xc1]);
    check(|b| b.add(R::rsp.into(), R::r12.into()), &[0x49, 0x03, 0xe4]);
    check(|b| b.add(R::r14.into(), R::r10.into()), &[0x4d, 0x03, 0xf2]);

    // reg, imm
    check(
        |b| b.add(R::rax.into(), 0i32.into()),
        &[0x48, 0x83, 0xc0, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), 0x7fi32.into()),
        &[0x48, 0x83, 0xc0, 0x7f],
    );
    check(
        |b| b.add(R::rax.into(), 0x80i32.into()),
        &[0x48, 0x81, 0xc0, 0x80, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r10.into(), 0x7fffffffi32.into()),
        &[0x49, 0x81, 0xc2, 0xff, 0xff, 0xff, 0x7f],
    );
    check(|b| b.add(R::al.into(), 3i32.into()), &[0x80, 0xc0, 0x03]);
    check(
        |b| b.add(R::sil.into(), 3i32.into()),
        &[0x40, 0x80, 0xc6, 0x03],
    );
    check(
        |b| b.add(R::r11b.into(), 3i32.into()),
        &[0x41, 0x80, 0xc3, 0x03],
    );

    // reg, [reg]
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax)),
        &[0x48, 0x03, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rbx)),
        &[0x48, 0x03, 0x03],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rsp)),
        &[0x48, 0x03, 0x04, 0x24],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rbp)),
        &[0x48, 0x03, 0x45, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r10)),
        &[0x49, 0x03, 0x02],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r12)),
        &[0x49, 0x03, 0x04, 0x24],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r13)),
        &[0x49, 0x03, 0x45, 0x00],
    );

    check(
        |b| b.add(R::r12.into(), idx(qword, R::rax)),
        &[0x4c, 0x03, 0x20],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rbx)),
        &[0x4c, 0x03, 0x23],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rsp)),
        &[0x4c, 0x03, 0x24, 0x24],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rbp)),
        &[0x4c, 0x03, 0x65, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r10)),
        &[0x4d, 0x03, 0x22],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r12)),
        &[0x4d, 0x03, 0x24, 0x24],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r13)),
        &[0x4d, 0x03, 0x65, 0x00],
    );

    // reg, [base+imm8]
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + 0x1b)),
        &[0x48, 0x03, 0x40, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rbx + 0x1b)),
        &[0x48, 0x03, 0x43, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rsp + 0x1b)),
        &[0x48, 0x03, 0x44, 0x24, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rbp + 0x1b)),
        &[0x48, 0x03, 0x45, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r10 + 0x1b)),
        &[0x49, 0x03, 0x42, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r12 + 0x1b)),
        &[0x49, 0x03, 0x44, 0x24, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r13 + 0x1b)),
        &[0x49, 0x03, 0x45, 0x1b],
    );

    check(
        |b| b.add(R::r12.into(), idx(qword, R::rax + 0x1b)),
        &[0x4c, 0x03, 0x60, 0x1b],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rbx + 0x1b)),
        &[0x4c, 0x03, 0x63, 0x1b],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rsp + 0x1b)),
        &[0x4c, 0x03, 0x64, 0x24, 0x1b],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rbp + 0x1b)),
        &[0x4c, 0x03, 0x65, 0x1b],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r10 + 0x1b)),
        &[0x4d, 0x03, 0x62, 0x1b],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r12 + 0x1b)),
        &[0x4d, 0x03, 0x64, 0x24, 0x1b],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r13 + 0x1b)),
        &[0x4d, 0x03, 0x65, 0x1b],
    );

    // reg, [base+imm32]
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + 0xabab)),
        &[0x48, 0x03, 0x80, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rbx + 0xabab)),
        &[0x48, 0x03, 0x83, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rsp + 0xabab)),
        &[0x48, 0x03, 0x84, 0x24, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rbp + 0xabab)),
        &[0x48, 0x03, 0x85, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r10 + 0xabab)),
        &[0x49, 0x03, 0x82, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r12 + 0xabab)),
        &[0x49, 0x03, 0x84, 0x24, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r13 + 0xabab)),
        &[0x49, 0x03, 0x85, 0xab, 0xab, 0x00, 0x00],
    );

    check(
        |b| b.add(R::r12.into(), idx(qword, R::rax + 0xabab)),
        &[0x4c, 0x03, 0xa0, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rbx + 0xabab)),
        &[0x4c, 0x03, 0xa3, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rsp + 0xabab)),
        &[0x4c, 0x03, 0xa4, 0x24, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rbp + 0xabab)),
        &[0x4c, 0x03, 0xa5, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r10 + 0xabab)),
        &[0x4d, 0x03, 0xa2, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r12 + 0xabab)),
        &[0x4d, 0x03, 0xa4, 0x24, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r13 + 0xabab)),
        &[0x4d, 0x03, 0xa5, 0xab, 0xab, 0x00, 0x00],
    );

    // reg, [index*scale]
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax * 2)),
        &[0x48, 0x03, 0x04, 0x45, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rbx * 2)),
        &[0x48, 0x03, 0x04, 0x5d, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rbp * 2)),
        &[0x48, 0x03, 0x04, 0x6d, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r10 * 2)),
        &[0x4a, 0x03, 0x04, 0x55, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r12 * 2)),
        &[0x4a, 0x03, 0x04, 0x65, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::r13 * 2)),
        &[0x4a, 0x03, 0x04, 0x6d, 0x00, 0x00, 0x00, 0x00],
    );

    check(
        |b| b.add(R::r12.into(), idx(qword, R::rax * 2)),
        &[0x4c, 0x03, 0x24, 0x45, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rbx * 2)),
        &[0x4c, 0x03, 0x24, 0x5d, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rbp * 2)),
        &[0x4c, 0x03, 0x24, 0x6d, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r10 * 2)),
        &[0x4e, 0x03, 0x24, 0x55, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r12 * 2)),
        &[0x4e, 0x03, 0x24, 0x65, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::r13 * 2)),
        &[0x4e, 0x03, 0x24, 0x6d, 0x00, 0x00, 0x00, 0x00],
    );

    // reg, [base+index*scale+imm]
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + R::rax * 2)),
        &[0x48, 0x03, 0x04, 0x40],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + R::rbx * 2 + 0x1b)),
        &[0x48, 0x03, 0x44, 0x58, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + R::rbp * 2)),
        &[0x48, 0x03, 0x04, 0x68],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + R::rbp + 0xabab)),
        &[0x48, 0x03, 0x84, 0x28, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + R::r12 + 0x1b)),
        &[0x4a, 0x03, 0x44, 0x20, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + R::r12 * 4 + 0xabab)),
        &[0x4a, 0x03, 0x84, 0xa0, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + R::r13 * 2 + 0x1b)),
        &[0x4a, 0x03, 0x44, 0x68, 0x1b],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, R::rax + R::r13 + 0xabab)),
        &[0x4a, 0x03, 0x84, 0x28, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rax + R::r12 * 2)),
        &[0x4e, 0x03, 0x24, 0x60],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rax + R::r13 + 0xabab)),
        &[0x4e, 0x03, 0xa4, 0x28, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(R::r12.into(), idx(qword, R::rax + R::rbp * 2 + 0x1b)),
        &[0x4c, 0x03, 0x64, 0x68, 0x1b],
    );

    // reg, [imm32]
    check(
        |b| b.add(R::rax.into(), idx(qword, 0)),
        &[0x48, 0x03, 0x04, 0x25, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.add(R::rax.into(), idx(qword, 0xabab)),
        &[0x48, 0x03, 0x04, 0x25, 0xab, 0xab, 0x00, 0x00],
    );

    // [addr], reg
    check(
        |b| b.add(idx(qword, R::rax), R::rax.into()),
        &[0x48, 0x01, 0x00],
    );
    check(
        |b| b.add(idx(qword, R::rax + R::rax * 4 + 0xabab), R::rax.into()),
        &[0x48, 0x01, 0x84, 0x80, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(idx(qword, R::rbx + R::rax * 2 + 0x1b), R::rax.into()),
        &[0x48, 0x01, 0x44, 0x43, 0x1b],
    );
    check(
        |b| b.add(idx(qword, R::rbx + R::rbp * 2 + 0x1b), R::rax.into()),
        &[0x48, 0x01, 0x44, 0x6b, 0x1b],
    );
    check(
        |b| b.add(idx(qword, R::rbp + R::rbp * 4 + 0xabab), R::rax.into()),
        &[0x48, 0x01, 0x84, 0xad, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(idx(qword, R::rbp + R::r12 + 0x1b), R::rax.into()),
        &[0x4a, 0x01, 0x44, 0x25, 0x1b],
    );
    check(
        |b| b.add(idx(qword, R::r12), R::rax.into()),
        &[0x49, 0x01, 0x04, 0x24],
    );
    check(
        |b| b.add(idx(qword, R::r13 + R::rbx + 0xabab), R::rax.into()),
        &[0x49, 0x01, 0x84, 0x1d, 0xab, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(idx(qword, R::rax + R::r13 * 2 + 0x1b), R::rsi.into()),
        &[0x4a, 0x01, 0x74, 0x68, 0x1b],
    );
    check(
        |b| b.add(idx(qword, R::rbp + R::rbx * 2), R::rsi.into()),
        &[0x48, 0x01, 0x74, 0x5d, 0x00],
    );
    check(
        |b| b.add(idx(qword, R::rsp + R::r10 * 2 + 0x1b), R::r10.into()),
        &[0x4e, 0x01, 0x54, 0x54, 0x1b],
    );

    // [addr], imm
    check(
        |b| b.add(idx(byte, R::rax), 2i32.into()),
        &[0x80, 0x00, 0x02],
    );
    check(
        |b| b.add(idx(dword, R::rax), 2i32.into()),
        &[0x83, 0x00, 0x02],
    );
    check(
        |b| b.add(idx(dword, R::rax), 0xabcdi32.into()),
        &[0x81, 0x00, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.add(idx(qword, R::rax), 2i32.into()),
        &[0x48, 0x83, 0x00, 0x02],
    );
    check(
        |b| b.add(idx(qword, R::rax), 0xabcdi32.into()),
        &[0x48, 0x81, 0x00, 0xcd, 0xab, 0x00, 0x00],
    );
}
