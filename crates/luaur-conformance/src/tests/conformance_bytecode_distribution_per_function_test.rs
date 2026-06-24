//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4435:conformance_bytecode_distribution_per_function_test`
//! Source: `tests/Conformance.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Conformance.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file Compiler/include/luacode.h
//!   - includes -> source_file CodeGen/include/luacodegen.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file CodeGen/include/Luau/BytecodeSummary.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file tests/ConformanceIrHooks.h
//! - incoming:
//!   - declares <- source_file tests/Conformance.test.cpp
//! - outgoing:
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record FunctionBytecodeSummary (CodeGen/include/Luau/BytecodeSummary.h)
//!   - calls -> method TypeError::summary (Analysis/src/Error.cpp)
//!   - calls -> method FunctionBytecodeSummary::getCounts (CodeGen/include/Luau/BytecodeSummary.h)
//!   - calls -> function analyzeFile (tests/Conformance.test.cpp)
//!   - calls -> method FunctionBytecodeSummary::getLine (CodeGen/include/Luau/BytecodeSummary.h)
//!   - calls -> method FunctionBytecodeSummary::getCount (CodeGen/include/Luau/BytecodeSummary.h)
//!   - translates_to -> rust_item conformance_bytecode_distribution_per_function_test

#[cfg(test)]
#[test]
fn conformance_bytecode_distribution_per_function_test() {
    use crate::functions::analyze_file::analyze_file;
    use luaur_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;
    use luaur_common::enums::luau_opcode::LuauOpcode;

    let source = r#"
local function first(n, p)
  local t = {}
  for i=1,p do t[i] = i*10 end

  local function inner(_,n)
    if n > 0 then
      n = n-1
      return n, unpack(t)
    end
  end
  return inner, nil, n
end

local function second(x)
 return x[1]
end
"#;

    let total_count =
        |summary: &FunctionBytecodeSummary| -> u32 { summary.get_counts(0).iter().copied().sum() };

    let summaries = analyze_file(source, 0, 1);

    assert_eq!("inner", summaries[0].get_name());
    assert_eq!(6, summaries[0].get_line());
    assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_LOADN as u8));
    assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_MOVE as u8));
    assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_GETUPVAL as u8));
    assert_eq!(
        1,
        summaries[0].get_count(0, LuauOpcode::LOP_GETIMPORT as u8)
    );
    assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_CALL as u8));
    assert_eq!(2, summaries[0].get_count(0, LuauOpcode::LOP_RETURN as u8));
    assert_eq!(
        1,
        summaries[0].get_count(0, LuauOpcode::LOP_JUMPIFNOTLT as u8)
    );
    assert_eq!(1, summaries[0].get_count(0, LuauOpcode::LOP_SUBK as u8));
    assert_eq!(
        1,
        summaries[0].get_count(0, LuauOpcode::LOP_FASTCALL1 as u8)
    );
    assert_eq!(10, total_count(&summaries[0]));

    assert_eq!("first", summaries[1].get_name());
    assert_eq!(2, summaries[1].get_line());
    assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_LOADNIL as u8));
    assert_eq!(2, summaries[1].get_count(0, LuauOpcode::LOP_LOADN as u8));
    assert_eq!(3, summaries[1].get_count(0, LuauOpcode::LOP_MOVE as u8));
    assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_SETTABLE as u8));
    assert_eq!(
        1,
        summaries[1].get_count(0, LuauOpcode::LOP_NEWCLOSURE as u8)
    );
    assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_RETURN as u8));
    assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_MULK as u8));
    assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_NEWTABLE as u8));
    assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_FORNPREP as u8));
    assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_FORNLOOP as u8));
    assert_eq!(1, summaries[1].get_count(0, LuauOpcode::LOP_CAPTURE as u8));
    assert_eq!(14, total_count(&summaries[1]));

    assert_eq!("second", summaries[2].get_name());
    assert_eq!(15, summaries[2].get_line());
    assert_eq!(
        1,
        summaries[2].get_count(0, LuauOpcode::LOP_GETTABLEN as u8)
    );
    assert_eq!(1, summaries[2].get_count(0, LuauOpcode::LOP_RETURN as u8));
    assert_eq!(2, total_count(&summaries[2]));

    assert_eq!("", summaries[3].get_name());
    assert_eq!(1, summaries[3].get_line());
    assert_eq!(1, summaries[3].get_count(0, LuauOpcode::LOP_RETURN as u8));
    assert_eq!(
        2,
        summaries[3].get_count(0, LuauOpcode::LOP_DUPCLOSURE as u8)
    );
    assert_eq!(
        1,
        summaries[3].get_count(0, LuauOpcode::LOP_PREPVARARGS as u8)
    );
    assert_eq!(4, total_count(&summaries[3]));
}
