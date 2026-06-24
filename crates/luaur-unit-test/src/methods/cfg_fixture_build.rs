impl crate::records::cfg_fixture::CfgFixture {
    pub fn build(
        &mut self,
        code: &str,
    ) -> *mut luaur_analysis::records::control_flow_graph::ControlFlowGraph {
        use luaur_analysis::functions::dump_cfg::dump_cfg;
        use luaur_analysis::functions::dump_cfg_json::dump_cfg_json;
        use luaur_analysis::records::cfg_builder::CfgBuilder;
        use luaur_common::FFlag;

        self.root = self.parse(code);

        let cfg = CfgBuilder::make_cfg(&mut self.cfg_allocator as *mut _, self.root);

        if FFlag::DebugLuauLogCFG.get() {
            print!("{}", dump_cfg(unsafe { &*cfg }));
        }

        if FFlag::DebugLuauDumpCFGJson.get() {
            println!("{}", dump_cfg_json(unsafe { &*cfg }));
        }

        cfg
    }
}
