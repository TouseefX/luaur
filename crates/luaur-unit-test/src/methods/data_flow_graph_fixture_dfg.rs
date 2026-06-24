use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
use luaur_analysis::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::parse_errors::ParseErrors;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;

impl DataFlowGraphFixture {
    pub fn dfg(&mut self, code: &str) {
        self.names.rebind_allocator(&mut self.allocator as *mut _);

        let result = Parser::parse(
            code,
            code.len(),
            &mut self.names,
            &mut self.allocator,
            ParseOptions::default(),
        );

        if !result.errors.is_empty() {
            std::panic::panic_any(ParseErrors::new(result.errors));
        }

        self.module = result.root;
        self.graph = Some(DataFlowGraphBuilder::build(
            self.module,
            &mut self.def_arena as *mut _,
            &mut self.key_arena as *mut _,
            &mut self.handle as *mut _,
        ));
    }
}
