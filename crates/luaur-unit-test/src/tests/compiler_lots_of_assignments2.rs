#[cfg(test)]
#[test]
fn compiler_lots_of_assignments2() {
    use alloc::string::String;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
    use luaur_compiler::records::compile_options::CompileOptions;

    let source = String::from("g01,g02,g03,g04,g05,g06,g07,g08,g09,g0a,g0b,g0c,g0d,g0e,g0f,g10,g11,g12,g13,g14,g15,g16,g17,g18,g19,g1a,g1b,g1c,g1d,g1e,g1f,g20,g21,g22,g23,g24,g25,g26,g27,g28,g29,g2a,g2b,g2c,g2d,g2e,g2f,g30,g31,g32,g33,g34,g35,g36,g37,g38,g39,g3a,g3b,g3c,g3d,g3e,g3f,g40,g41,g42,g43,g44,g45,g46,g47,g48,g49,g4a,g4b,g4c,g4d,g4e,g4f,g50,g51,g52,g53,g54,g55,g56,g57,g58,g59,g5a,g5b,g5c,g5d,g5e,g5f,g60,g61,g62,g63,g64,g65,g66,g67,g68,g69,g6a,g6b,g6c,g6d,g6e,g6f,g70,g71,g72,g73,g74,g75,g76,g77,g78,g79,g7a,g7b,g7c,g7d,g7e,g7f,g80,g81,g82,g83,g84,g85,g86,g87,g88,g89,g8a,g8b,g8c,g8d,g8e,g8f,g90,g91,g92,g93,g94,g95,g96,g97,g98,g99,g9a,g9b,g9c,g9d,g9e,g9f,ga0,ga1,ga2,ga3,ga4,ga5,ga6,ga7,ga8,ga9,gaa,gab,gac,gad,gae,gaf,gb0,gb1,gb2,gb3,gb4,gb5,gb6,gb7,gb8,gb9,gba,gbb,gbc,gbd,gbe,gbf,gc0,gc1,gc2,gc3,gc4,gc5,gc6,gc7,gc8,gc9,gca,gcb,gcc,gcd,gce,gcf,gd0,gd1,gd2,gd3,gd4,gd5,gd6,gd7,gd8,gd9,gda,gdb,gdc,gdd,gde,gdf,ge0,ge1,ge2,ge3,ge4,ge5,ge6,ge7,ge8,ge9,gea,geb,gec,ged,gee,gef,gf0,gf1,gf2,gf3,gf4,gf5,gf6,gf7,gf8,gf9,gfa,gfb,gfc,gfd,gfe,gff,g00 = ...");

    let mut bcb = BytecodeBuilder::new(None);
    let options = CompileOptions::default();
    let parse_options = ParseOptions::default();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        compile_or_throw_bytecode_builder_string_compile_options_parse_options(
            &mut bcb,
            &source,
            &options,
            &parse_options,
        );
    }));

    assert!(result.is_err(), "Expected exception");
    let err = result.unwrap_err();
    let msg = err
        .downcast_ref::<luaur_compiler::records::compile_error::CompileError>()
        .map(|e| alloc::format!("{e}"))
        .or_else(|| err.downcast_ref::<String>().cloned())
        .or_else(|| {
            err.downcast_ref::<&'static str>()
                .map(|s| alloc::string::ToString::to_string(s))
        })
        .unwrap_or_default();
    assert_eq!(
        msg,
        "Out of registers when trying to allocate 256 registers: exceeded limit 255"
    );
}
