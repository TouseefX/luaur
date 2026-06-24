#[cfg(test)]
#[test]
fn compiler_debug_locals() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
    use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;

    let _emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    let source = String::from(
        "\nfunction foo(e, f)\n\
         local a = 1\n\
         for i=1,3 do\n\
             print(i)\n\
         end\n\
         for k,v in pairs() do\n\
             print(k, v)\n\
         end\n\
         do\n\
             local b = 2\n\
             print(b)\n\
         end\n\
         do\n\
             local c = 2\n\
             print(b)\n\
         end\n\
         local function inner()\n\
             return inner, a\n\
         end\n\
         return a\n\
     end\n",
    );

    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(
        BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES | BytecodeBuilder::DUMP_LOCALS,
    );
    bcb.set_dump_source(&source);

    let mut options = luaur_compiler::records::compile_options::CompileOptions::default();
    options.optimization_level = 1;
    options.debug_level = 2;
    options.type_info_level = 0;
    options.coverage_level = 0;

    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );

    let dump_func = bcb.dump_function(1);
    let expected_func = "\n\
local 0: reg 5, start pc 5 line 5, end pc 9 line 5\n\
local 1: reg 6, start pc 16 line 8, end pc 21 line 8\n\
local 2: reg 7, start pc 16 line 8, end pc 21 line 8\n\
local 3: reg 3, start pc 25 line 12, end pc 29 line 12\n\
local 4: reg 3, start pc 31 line 16, end pc 36 line 16\n\
local 5: reg 0, start pc 0 line 3, end pc 40 line 21\n\
local 6: reg 1, start pc 0 line 3, end pc 40 line 21\n\
local 7: reg 2, start pc 1 line 4, end pc 40 line 21\n\
local 8: reg 3, start pc 40 line 21, end pc 40 line 21\n\
3: LOADN R2 1\n\
4: LOADN R5 1\n\
4: LOADN R3 3\n\
4: LOADN R4 1\n\
4: FORNPREP R3 L1\n\
5: L0: GETIMPORT R6 1 [print]\n\
5: MOVE R7 R5\n\
5: CALLFB R6 1 0 [0]\n\
4: FORNLOOP R3 L0\n\
7: L1: GETIMPORT R3 3 [pairs]\n\
7: CALLFB R3 0 3 [1]\n\
7: FORGPREP_NEXT R3 L3\n\
8: L2: GETIMPORT R8 1 [print]\n\
8: MOVE R9 R6\n\
8: MOVE R10 R7\n\
8: CALLFB R8 2 0 [2]\n\
7: L3: FORGLOOP R3 L2 2\n\
11: LOADN R3 2\n\
12: GETIMPORT R4 1 [print]\n\
12: LOADN R5 2\n\
12: CALLFB R4 1 0 [3]\n\
15: LOADN R3 2\n\
16: GETIMPORT R4 1 [print]\n\
16: GETIMPORT R5 5 [b]\n\
16: CALLFB R4 1 0 [4]\n\
18: NEWCLOSURE R3 P0\n\
18: CAPTURE VAL R3\n\
18: CAPTURE VAL R2\n\
21: RETURN R2 1\n";

    assert_eq!("\n".to_string() + &dump_func, expected_func);
}
