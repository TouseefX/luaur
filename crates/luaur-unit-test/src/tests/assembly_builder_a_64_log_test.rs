#[cfg(test)]
#[test]
fn assembly_builder_a_64_log_test() {
    use luaur_code_gen::enums::address_kind_a_64::AddressKindA64;
    use luaur_code_gen::enums::condition_a_64::ConditionA64;
    use luaur_code_gen::records::address_a_64::AddressA64;
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::label::Label;
    use luaur_code_gen::records::register_a_64::RegisterA64 as R;
    use luaur_code_gen::type_aliases::mem::mem;

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(true, 0);

    build.add_register_a_64_register_a_64_u16(R::sp, R::sp, 4);
    build.add_register_a_64_register_a_64_register_a_64_i32(R::w0, R::w1, R::w2, 0);
    build.add_register_a_64_register_a_64_register_a_64_i32(R::x0, R::x1, R::x2, 2);
    build.add_register_a_64_register_a_64_register_a_64_i32(R::x0, R::x1, R::x2, -2);
    build.add_register_a_64_register_a_64_u16(R::w7, R::w8, 5);
    build.add_register_a_64_register_a_64_u16(R::x7, R::x8, 5);
    build.ldr(R::x7, mem(R::x8, 0));
    build.ldr(R::x7, mem(R::x8, 8));
    build.ldr(R::x7, mem(R::x8, R::x9));
    build.mov_register_a_64_register_a_64(R::x1, R::x2);
    build.movk(R::x1, 42, 16);
    build.cmp_register_a_64_register_a_64(R::x1, R::x2);
    build.blr(R::x0);

    let mut l = Label::default();
    build.b_condition_a_64_label(ConditionA64::Plus, &mut l);
    build.cbz(R::x7, &mut l);

    build.ldp(R::x0, R::x1, mem(R::x8, 8));
    build.adr_register_a_64_label(R::x0, &mut l);

    build.fabs(R::d1, R::d2);
    build.ldr(R::q1, mem(R::x2, 0));

    build.csel(R::x0, R::x1, R::x2, ConditionA64::Equal);
    build.cset(R::x0, ConditionA64::Equal);

    build.fcmp(R::d0, R::d1);
    build.fcmpz(R::d0);

    build.fmov_register_a_64_f64(R::d0, 0.25);
    build.tbz(R::x0, 5, &mut l);

    build.fcvt(R::s1, R::d2);

    build.ubfx(R::x1, R::x2, 37, 5);

    build.ldr(R::x0, mem(R::x1, 1));
    build.ldr(
        R::x0,
        AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(R::x1, 1, AddressKindA64::pre),
    );
    build.ldr(
        R::x0,
        AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            R::x1,
            1,
            AddressKindA64::post,
        ),
    );

    build.add_register_a_64_register_a_64_register_a_64_i32(R::x1, R::x2, R::w3, 3);

    build.ins_4_s_register_a_64_register_a_64_u8(R::q29, R::w17, 3);
    build.ins_4_s_register_a_64_u8_register_a_64_u8(R::q31, 1, R::q29, 2);
    build.dup_4s(R::s29, R::q31, 2);
    build.dup_4s(R::q29, R::q30, 0);
    build.umov_4s(R::w1, R::q30, 3);
    build.fmul(R::q0, R::q1, R::q2);

    build.fcmeq_4s(R::q2, R::q0, R::q1);
    build.bit(R::q1, R::q0, R::q2);

    build.set_label_label(&mut l);
    build.ret();

    build.finalize();

    let expected = "\n add         sp,sp,#4\n add         w0,w1,w2\n add         x0,x1,x2 LSL #2\n add         x0,x1,x2 LSR #2\n add         w7,w8,#5\n add         x7,x8,#5\n ldr         x7,[x8]\n ldr         x7,[x8,#8]\n ldr         x7,[x8,x9]\n mov         x1,x2\n movk        x1,#42 LSL #16\n cmp         x1,x2\n blr         x0\n b.pl        .L1\n cbz         x7,.L1\n ldp         x0,x1,[x8,#8]\n adr         x0,.L1\n fabs        d1,d2\n ldr         q1,[x2]\n csel        x0,x1,x2,eq\n cset        x0,eq\n fcmp        d0,d1\n fcmp        d0,#0\n fmov        d0,#0.25\n tbz         x0,#5,.L1\n fcvt        s1,d2\n ubfx        x1,x2,#3705\n ldr         x0,[x1,#1]\n ldr         x0,[x1,#1]!\n ldr         x0,[x1]!,#1\n add         x1,x2,w3 UXTW #3\n ins         v29.s[3],w17\n ins         v31.s[1],v29.s[2]\n dup         s29,v31.s[2]\n dup         v29.4s,v30.s[0]\n umov        w1,v30.s[3]\n fmul        v0.4s,v1.4s,v2.4s\n fcmeq       v2.4s,v0.4s,v1.4s\n bit         v1.16b,v0.16b,v2.16b\n.L1:\n ret\n";

    assert_eq!(
        format!("\n{}", build.text),
        expected,
        "disasm text mismatch"
    );
}
