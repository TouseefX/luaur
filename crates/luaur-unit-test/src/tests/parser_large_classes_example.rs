#[cfg(test)]
#[test]
fn parser_large_classes_example() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::parse_options::ParseOptions;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = alloc::string::String::from(
        "\n        class PlayerStats\n            public name: string\n            public health: number\n            public level: number\n\n            -- Static 'Constructor'\n            function new(name: string)\n                return PlayerStats {\n                    name = name,\n                    health = 100,\n                    level = 1\n                }\n            end\n\n            -- Method\n            function heal(self, amount: number)\n                self.health = math.min(100, self.health + amount)\n                print(self.name .. \" healed to \" .. self.health)\n            end\n\n            -- Metamethod for printing\n            function __tostring(self)\n                return self.name .. \" (Level \" .. self.level .. \") - Health: \" .. self.health\n            end\n        end\n\n        local player = PlayerStats.new(\"John Doe\")\n        print(player.name)\n        player:heal(20)\n        print(player.name)\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert_eq!(result.errors.len(), 0);
}
