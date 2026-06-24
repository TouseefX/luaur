//! Compile Luau source to bytecode (without running it).
//!
//!     cargo run -p luaur-example-compile-to-bytecode

fn main() {
    let source = "local x = 41\nreturn x + 1";

    match luaur::compile(source) {
        Ok(bytecode) => {
            // `bytecode` is the same blob `luau_load` consumes. The first byte is
            // the bytecode version target.
            println!("compiled {} bytes of bytecode", bytecode.len());
            println!("version byte: {}", bytecode.first().copied().unwrap_or(0));
        }
        Err(message) => {
            // A parse/compile error is returned as the human-readable message.
            eprintln!("compile error: {message}");
            std::process::exit(1);
        }
    }

    // Syntax errors surface as `Err`, not a panic:
    match luaur::compile("local = = 3") {
        Ok(_) => unreachable!("that should not have compiled"),
        Err(message) => println!("(expected) syntax error: {message}"),
    }
}
