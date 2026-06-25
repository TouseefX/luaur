//! Source: `CLI/src/Ast.cpp:24-90` (hand-ported)
use crate::functions::assertion_handler::assertion_handler;
use crate::functions::display_help::display_help;
use luaur_analysis::functions::to_json_ast_json_encoder_alt_b::to_json;
use luaur_analysis::functions::to_string_to_string_alt_t::to_string_location_i32_bool;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;
use luaur_cli_lib::functions::read_file::read_file;
use luaur_cli_lib::functions::read_stdin::read_stdin;
use luaur_cli_lib::functions::set_luau_flags_default::set_luau_flags_default;

/// C++ `int main(int argc, char** argv)` (`CLI/src/Ast.cpp:24-90`).
pub fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let args: alloc::vec::Vec<alloc::string::String> = std::env::args().collect();
    let argc = args.len();

    // Luau::assertHandler() = assertionHandler;
    *luaur_common::functions::assert_handler::assert_handler() = Some(assertion_handler);

    // for (FValue<bool>* flag = ...; flag; flag = flag->next)
    //     if (strncmp(flag->name, "Luau", 4) == 0) flag->value = true;
    // The shared port enables every `Luau`-prefixed flag (matching the C++ loop;
    // it additionally skips experimental flags, the project's CLI convention).
    set_luau_flags_default();

    // if (argc >= 2 && strcmp(argv[1], "--help") == 0) { displayHelp(argv[0]); return 0; }
    if argc >= 2 && args[1] == "--help" {
        display_help(&args[0]);
        return 0;
    }
    // else if (argc < 2) { displayHelp(argv[0]); return 1; }
    else if argc < 2 {
        display_help(&args[0]);
        return 1;
    }

    // const char* name = argv[1];
    let name = &args[1];

    // std::optional<std::string> maybeSource;
    // if (strcmp(name, "-") == 0) maybeSource = readStdin(); else maybeSource = readFile(name);
    let maybe_source = if name == "-" {
        read_stdin()
    } else {
        read_file(name)
    };

    // if (!maybeSource) { fprintf(stderr, "Couldn't read source %s\n", name); return 1; }
    let source = match maybe_source {
        Some(s) => s,
        None => {
            eprintln!("Couldn't read source {}", name);
            return 1;
        }
    };

    // Luau::Allocator allocator; Luau::AstNameTable names(allocator);
    // The `AstNameTable` keeps a `*mut Allocator`, so both are boxed for a stable
    // address (mirrors the parser test `Fixture`).
    let mut allocator = alloc::boxed::Box::new(Allocator::allocator());
    let mut names = alloc::boxed::Box::new(AstNameTable::new(&mut allocator));

    // ParseOptions options; options.captureComments = true; options.allowDeclarationSyntax = true;
    let mut options = ParseOptions::default();
    options.capture_comments = true;
    options.allow_declaration_syntax = true;

    // ParseResult parseResult = Parser::parse(source.data(), source.size(), names, allocator, std::move(options));
    let parse_result = Parser::parse(&source, source.len(), &mut names, &mut allocator, options);

    // if (parseResult.errors.size() > 0) { ... print each error ... }
    if !parse_result.errors.is_empty() {
        eprintln!("Parse errors were encountered:");
        for error in &parse_result.errors {
            eprintln!(
                "  {} - {}",
                to_string_location_i32_bool(error.get_location(), 0, true),
                error.get_message()
            );
        }
        eprintln!();
    }

    // printf("%s", Luau::toJson(parseResult.root, parseResult.commentLocations).c_str());
    let json = to_json(
        parse_result.root as *mut AstNode,
        parse_result.comment_locations.clone(),
    );
    print!("{}", json);

    // return parseResult.errors.size() > 0 ? 1 : 0;
    if parse_result.errors.is_empty() {
        0
    } else {
        1
    }
}
