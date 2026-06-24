use crate::records::frontend::Frontend;
use crate::records::source_module::SourceModule;
use crate::type_aliases::module_name_file_resolver::ModuleName;
use alloc::sync::Arc;
use luaur_ast::enums::mode::Mode;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::location::Location;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;
use luaur_common::macros::luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

impl Frontend {
    pub fn parse_module_name_string_view_parse_options(
        &mut self,
        name: &ModuleName,
        src: &str,
        parse_options: &ParseOptions,
    ) -> SourceModule {
        LUAU_TIMETRACE_SCOPE!("Frontend::parse", "Frontend");
        LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

        let mut source_module = SourceModule::source_module();

        let timestamp = crate::functions::get_timestamp::get_timestamp();

        let parse_result = Parser::parse(
            src,
            src.len(),
            Arc::get_mut(&mut source_module.names)
                .expect("SourceModule names must be uniquely owned while parsing"),
            Arc::get_mut(&mut source_module.allocator)
                .expect("SourceModule allocator must be uniquely owned while parsing"),
            parse_options.clone(),
        );

        self.stats.time_parse += crate::functions::get_timestamp::get_timestamp() - timestamp;
        self.stats.files += 1;
        self.stats.lines += parse_result.lines;

        if !parse_result.errors.is_empty() {
            source_module
                .parse_errors
                .extend(parse_result.errors.iter().cloned());
        }

        if parse_result.errors.is_empty() || !parse_result.root.is_null() {
            source_module.root = parse_result.root;
            source_module.mode =
                crate::functions::parse_mode::parse_mode(&parse_result.hotcomments);
        } else {
            let empty_body = AstArray {
                data: core::ptr::null_mut(),
                size: 0,
            };
            source_module.root = Arc::get_mut(&mut source_module.allocator)
                .expect("SourceModule allocator must be uniquely owned while parsing")
                .alloc(AstStatBlock::new(
                    Location::new(
                        luaur_ast::records::position::Position::default(),
                        luaur_ast::records::position::Position::default(),
                    ),
                    empty_body,
                    false,
                ));
            source_module.mode = Some(Mode::NoCheck);
        }

        source_module.name = name.clone();
        source_module.human_readable_name =
            self.module_resolver.get_human_readable_module_name(name);

        if parse_options.capture_comments {
            source_module.comment_locations = parse_result.comment_locations;
            source_module.hotcomments = parse_result.hotcomments;
        }

        source_module
    }
}
