//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/BytecodeCompiler.test.cpp:849:bytecode_compiler_classes_bytecode_roundtrips`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method BytecodeCompilerFixture::checkRoundtrip (tests/BytecodeCompiler.test.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item bytecode_compiler_classes_bytecode_roundtrips

#[cfg(test)]
#[test]
fn bytecode_compiler_classes_bytecode_roundtrips() {
    use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _classes = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);
    let mut fixture = BytecodeCompilerFixture::new();
    fixture.check_roundtrip(
        r#"
        class Point
            public x
            public y

            function magnitude(self)
                return math.sqrt(self.x * self.x + self.y * self.y)
            end

            function __mul(self, other)
                return Point { x = self.x * other.x, y = self.y * other.y }
            end

            function __add(self, other)
                return Point { x = self.x + other.x, y = self.y + other.y }
            end

            function __eq(self, other)
                return self.x == other.x and self.y == other.y
            end

            function zero()
                return Point { x = 0, y = 0 }
            end

            function asserttriple(self)
                local mag = self:magnitude()
                assert(mag == math.ceil(mag), "Not a pythagorean triple!")
            end

            function __tostring(self)
                return `Point(x={self.x}, y={self.y})`
            end

        end

        print(Point)

        return { Point = Point }
    "#,
    );
}
