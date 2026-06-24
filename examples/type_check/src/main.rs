//! Type-check a Luau script with luaur — completing the compile/eval/check trio.
//!
//!     cargo run -p luaur-example-type-check

fn main() {
    // `check` type-checks the source (on the validated old solver). It returns
    // `Ok(())` when the source type-checks clean, or `Err` carrying one
    // "line: message" diagnostic per type error (1-based line numbers).
    let clean = r#"
        local function add(a: number, b: number): number
            return a + b
        end

        local total: number = add(2, 3)
        print(total)
    "#;

    match luaur::check(clean) {
        Ok(()) => println!("clean snippet: type-checks clean"),
        Err(errors) => {
            println!("clean snippet: unexpected diagnostics:");
            for err in &errors {
                println!("  {err}");
            }
        }
    }

    // A deliberately bad-typed snippet: assigning a string to a `number`.
    let bad = r#"
        local x: number = "not a number"
        print(x)
    "#;

    match luaur::check(bad) {
        Ok(()) => println!("bad snippet: unexpectedly type-checked clean"),
        Err(errors) => {
            println!("bad snippet: {} type error(s):", errors.len());
            for err in &errors {
                println!("  {err}");
            }
        }
    }
}
