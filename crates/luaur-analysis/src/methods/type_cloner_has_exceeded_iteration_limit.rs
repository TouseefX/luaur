use crate::records::type_cloner::TypeCloner;
use luaur_common::FInt;

impl TypeCloner {
    pub fn has_exceeded_iteration_limit(&self) -> bool {
        if FInt::LuauTypeCloneIterationLimit.get() == 0 {
            return false;
        }

        self.steps as usize + self.queue.len() >= FInt::LuauTypeCloneIterationLimit.get() as usize
    }
}
