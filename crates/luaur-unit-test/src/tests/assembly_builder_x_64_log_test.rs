#[cfg(test)]
#[test]
fn assembly_builder_x_64_log_test() {
    use luaur_code_gen::enums::alignment_data_x_64::AlignmentDataX64;
    use luaur_code_gen::enums::condition_x_64::ConditionX64;
    use luaur_code_gen::enums::rounding_mode_x_64::RoundingModeX64;
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::label::Label;
    use luaur_code_gen::records::operand_x_64::{
        addr, byte, dword, qword, word, xmmword, ymmword, OperandX64,
    };
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    // `size[addr]` — the `[]` stamps the size prefix onto the address expression.
    fn idx(prefix: OperandX64, address: OperandX64) -> OperandX64 {
        prefix.operator_bracket(address)
    }

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(true, 0); // logText = true

    build.push(R::r12.into());
    build.align(8, AlignmentDataX64::Nop);
    build.align(8, AlignmentDataX64::Int3);
    build.align(8, AlignmentDataX64::Ud2);

    build.add(R::rax.into(), R::rdi.into());
    build.add(R::rcx.into(), 8i32.into());
    build.sub(idx(dword, R::rax.into()), 0x1fdci32.into());
    build.and_(idx(dword, R::rcx.into()), 0x37i32.into());
    build.mov(R::rdi.into(), idx(qword, R::rax + R::rsi * 2));
    build.vaddss(
        R::xmm0.into(),
        R::xmm0.into(),
        idx(dword, R::rax + R::r14 * 2 + 0x1c),
    );

    // C++ `Label start = build.setLabel();` — no-arg form: fresh label, set here.
    let mut start = Label::default();
    build.set_label(&mut start);
    build.cmp(R::rsi.into(), R::rdi.into());
    build.jcc(ConditionX64::Equal, &mut start);
    build.lea_register_x_64_label(R::rcx, &mut start);
    build.lea_operand_x_64_operand_x_64(R::rcx.into(), idx(addr, R::rdx.into()));

    build.jmp_operand_x_64(idx(qword, R::rdx.into()));
    build.vaddps(R::ymm9.into(), R::ymm12.into(), idx(ymmword, R::rbp + 0xc));
    let c = build.f64(2.5);
    build.vaddpd(R::ymm2.into(), R::ymm7.into(), c);
    build.neg(idx(qword, R::rbp + R::r12 * 2));
    build.mov64(R::r10, 0x1234567812345678i64);
    build.vmovapd(idx(xmmword, R::rax.into()), R::xmm11.into());
    build.movzx(R::eax, idx(byte, R::rcx.into()));
    build.movsx(R::rsi, idx(word, R::r12.into()));
    build.imul_operand_x_64_operand_x_64(R::rcx.into(), R::rdx.into());
    build.imul_operand_x_64_operand_x_64_i32(R::rcx.into(), R::rdx.into(), 8);
    build.vroundsd(
        R::xmm1.into(),
        R::xmm2.into(),
        R::xmm3.into(),
        RoundingModeX64::RoundToNearestEven,
    );
    build.vroundps(
        R::xmm1.into(),
        R::xmm12.into(),
        RoundingModeX64::RoundToNegativeInfinity,
    );
    build.add(R::rdx.into(), idx(qword, R::rcx - 12));
    build.pop(R::r12.into());
    build.cmov(ConditionX64::AboveEqual, R::rax, R::rbx.into());
    build.vpextrd(R::ecx, R::xmm5, 2);
    build.ret();
    build.int3();

    build.nop(1);
    build.nop(2);
    build.nop(3);
    build.nop(4);
    build.nop(5);
    build.nop(6);
    build.nop(7);
    build.nop(8);
    build.nop(9);

    build.finalize();

    let expected = "\n push        r12\n; align 8\n nop         word ptr[rax+rax] ; 6-byte nop\n; align 8 using int3\n; align 8 using ud2\n add         rax,rdi\n add         rcx,8\n sub         dword ptr [rax],1FDCh\n and         dword ptr [rcx],37h\n mov         rdi,qword ptr [rax+rsi*2]\n vaddss      xmm0,xmm0,dword ptr [rax+r14*2+01Ch]\n.L1:\n cmp         rsi,rdi\n je          .L1\n lea         rcx,.L1\n lea         rcx,[rdx]\n jmp         qword ptr [rdx]\n vaddps      ymm9,ymm12,ymmword ptr [rbp+0Ch]\n vaddpd      ymm2,ymm7,qword ptr [.start-8]\n neg         qword ptr [rbp+r12*2]\n mov         r10,1234567812345678h\n vmovapd     xmmword ptr [rax],xmm11\n movzx       eax,byte ptr [rcx]\n movsx       rsi,word ptr [r12]\n imul        rcx,rdx\n imul        rcx,rdx,8\n vroundsd    xmm1,xmm2,xmm3,8\n vroundps    xmm1,xmm12,9\n add         rdx,qword ptr [rcx-0Ch]\n pop         r12\n cmovae      rax,rbx\n vpextrd     ecx,xmm5,2\n ret\n int3\n nop\n xchg        ax, ax ; 2-byte nop\n nop         dword ptr[rax] ; 3-byte nop\n nop         dword ptr[rax] ; 4-byte nop\n nop         dword ptr[rax+rax] ; 5-byte nop\n nop         word ptr[rax+rax] ; 6-byte nop\n nop         dword ptr[rax] ; 7-byte nop\n nop         dword ptr[rax+rax] ; 8-byte nop\n nop         word ptr[rax+rax] ; 9-byte nop\n";

    assert_eq!(
        format!("\n{}", build.text),
        expected,
        "disasm text mismatch"
    );
}
