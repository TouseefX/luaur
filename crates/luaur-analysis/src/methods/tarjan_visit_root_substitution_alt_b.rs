use crate::enums::tarjan_result::TarjanResult;
use crate::records::tarjan::Tarjan;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::FInt;

impl Tarjan {
    pub fn visit_root_type_pack_id(&mut self, tp: TypePackId) -> TarjanResult {
        self.child_count = 0;
        if self.child_limit == 0 {
            self.child_limit = FInt::LuauTarjanChildLimit.get();
        }

        let tp = unsafe { (*self.log).follow_type_pack_id(tp) };

        let (index, _fresh) = self.indexify_type_pack_id(tp);
        self.worklist.push(
            crate::records::tarjan_worklist_vertex::TarjanWorklistVertex {
                index,
                curr_edge: -1,
                last_edge: -1,
            },
        );

        self.loop_item()
    }
}
