#[cfg(test)]
#[test]
fn assembly_builder_a_64_fp_math() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), expected: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], expected, "instruction word mismatch");
    }

    let d1 = RegisterA64::d1;
    let d2 = RegisterA64::d2;
    let d3 = RegisterA64::d3;
    let s1 = RegisterA64::s1;
    let s2 = RegisterA64::s2;
    let s29 = RegisterA64::s29;
    let s28 = RegisterA64::s28;
    let q1 = RegisterA64::q1;
    let q2 = RegisterA64::q2;
    let q29 = RegisterA64::q29;
    let q28 = RegisterA64::q28;
    let w1 = RegisterA64::w1;
    let w2 = RegisterA64::w2;
    let x1 = RegisterA64::x1;
    let x2 = RegisterA64::x2;
    let s30 = RegisterA64::s30;
    let q30 = RegisterA64::q30;
    let d28 = RegisterA64::d28;
    let d29 = RegisterA64::d29;

    check(|b| b.fabs(d1, d2), 0x1E60C041);
    check(|b| b.fabs(s1, s2), 0x1E20C041);
    check(|b| b.fabs(q1, q2), 0x4EA0F841);
    check(|b| b.fadd(d1, d2, d3), 0x1E632841);
    check(|b| b.fadd(s29, s29, s28), 0x1E3C2BBD);
    check(|b| b.fadd(q29, q29, q28), 0x4E3CD7BD);
    check(|b| b.fdiv(d1, d2, d3), 0x1E631841);
    check(|b| b.fdiv(s29, s29, s28), 0x1E3C1BBD);
    check(|b| b.fdiv(q29, q29, q28), 0x6E3CFFBD);
    check(|b| b.fmul(d1, d2, d3), 0x1E630841);
    check(|b| b.fmul(s29, s29, s28), 0x1E3C0BBD);
    check(|b| b.fmul(q29, q29, q28), 0x6E3CDFBD);
    check(|b| b.fneg(d1, d2), 0x1E614041);
    check(|b| b.fneg(s30, s30), 0x1E2143DE);
    check(|b| b.fneg(q30, q30), 0x6EA0FBDE);
    check(|b| b.fsqrt(d1, d2), 0x1E61C041);
    check(|b| b.fsub(d1, d2, d3), 0x1E633841);
    check(|b| b.fsub(s29, s29, s28), 0x1E3C3BBD);
    check(|b| b.fsub(q29, q29, q28), 0x4EBCD7BD);

    check(|b| b.faddp(s29, s28), 0x7E30DB9D);
    check(|b| b.faddp(d29, d28), 0x7E70DB9D);

    check(|b| b.frinta(d1, d2), 0x1E664041);
    check(|b| b.frintm(d1, d2), 0x1E654041);
    check(|b| b.frintp(d1, d2), 0x1E64C041);

    check(|b| b.frinta(s1, s2), 0x1E264041);
    check(|b| b.frintm(s1, s2), 0x1E254041);
    check(|b| b.frintp(s1, s2), 0x1E24C041);

    check(|b| b.frinta(q1, q2), 0x6E218841);
    check(|b| b.frintm(q1, q2), 0x4E219841);
    check(|b| b.frintp(q1, q2), 0x4EA18841);

    check(|b| b.fcvt(s1, d2), 0x1E624041);
    check(|b| b.fcvt(d1, s2), 0x1E22C041);

    check(|b| b.fcvtzs(w1, d2), 0x1E780041);
    check(|b| b.fcvtzs(x1, d2), 0x9E780041);
    check(|b| b.fcvtzu(w1, d2), 0x1E790041);
    check(|b| b.fcvtzu(x1, d2), 0x9E790041);

    check(|b| b.scvtf(d1, w2), 0x1E620041);
    check(|b| b.scvtf(d1, x2), 0x9E620041);

    check(|b| b.ucvtf(d1, w2), 0x1E630041);
    check(|b| b.ucvtf(d1, x2), 0x9E630041);
    check(|b| b.ucvtf(s1, w2), 0x1E230041);
    check(|b| b.ucvtf(s1, x2), 0x9E230041);

    // upstream passes A64::Feature_JSCVT to check() for this instruction
    use luaur_code_gen::enums::features_a_64::FeaturesA64;
    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(
        false,
        FeaturesA64::Feature_JSCVT as u32,
    );
    build.fjcvtzs(w1, d2);
    build.finalize();
    assert_eq!(
        build.code[0], 0x1E7E0041,
        "instruction word mismatch for fjcvtzs"
    );
}
