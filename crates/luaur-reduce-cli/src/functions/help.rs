/// Source: `CLI/src/Reduce.cpp:476-481` (`help`).
///
/// C++:
/// ```cpp
/// printf("Syntax: %s script command \"search text\"\n", args[0].data());
/// printf("    Within command, use {} as a stand-in for the script being reduced\n");
/// exit(1);
/// ```
/// `args[0]` is the program name; print it directly (the C++ `string_view::data()`
/// is the NUL-terminated `argv[0]`).
pub fn help(args: &[&str]) -> ! {
    let program_name = args.first().copied().unwrap_or("luaur-reduce");
    println!("Syntax: {} script command \"search text\"", program_name);
    println!("    Within command, use {{}} as a stand-in for the script being reduced");
    std::process::exit(1);
}
