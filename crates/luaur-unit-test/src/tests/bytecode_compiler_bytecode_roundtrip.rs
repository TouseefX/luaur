//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:768:bytecode_compiler_bytecode_roundtrip`
//! Source: `tests/BytecodeCompiler.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/BytecodeCompiler.test.cpp
//! - source_includes:
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeGraph.h
//!   - includes -> source_file Common/include/Luau/BytecodeWire.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/BytecodeCompiler.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method BytecodeCompilerFixture::checkRoundtrip (tests/BytecodeCompiler.test.cpp)
//!   - translates_to -> rust_item bytecode_compiler_bytecode_roundtrip

#[cfg(test)]
#[test]
fn bytecode_compiler_bytecode_roundtrip() {
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;

    let snippets = [
        r#"
        function fn(a, b)
            local extra = 0
            if a > b then extra = 1 end
            return extra + a + b
        end
    "#,
        r#"
        function fn()
            local var = 0
            repeat var += 1 until var < 10
        end
    "#,
        r#"
        function fn()
            local var = 3
            for i = 1, 10 do
                if var > 0 then print(i) end
                var -= 1;
            end
        end
    "#,
        r#"
        function fn()
            local res = 0
            local var = 0
            repeat
                local i = 0
                repeat
                    res += i * var
                    i += 1
                until i < 5
                var += 1
            until var < 10
        end
    "#,
        r#"
        local function x()
            local a, b = f()
            return b, a
        end
    "#,
        r#"
        local function fn(n)
            if n > 0 then
                return 0, 1
            else
                local a, b = fn(n - 1)
                return a + b, fn(n)
            end
        end
    "#,
        r#"
        local function fn(a, ...)
            local b, c = ...
            local l = {...}
            return a + b + c + l[1], ...
        end
    "#,
        r#"
        local function fn(x)
            local f = function (a, b) return a .. " and " .. b .. " and agian " .. b end
            return f(x, "eleven")
        end
    "#,
        r#"
        local tt = {}
        local function fn(x)
            local t = { a = x, b = x .. 42 }
            return table.insert({t}, tt)
        end
    "#,
    ];

    let mut fixture = BytecodeCompilerFixture::new();
    for snippet in snippets {
        fixture.check_roundtrip(snippet);
    }
}
