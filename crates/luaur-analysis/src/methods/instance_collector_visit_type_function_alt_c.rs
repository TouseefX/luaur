use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::instance_collector::InstanceCollector;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ffi::c_void;

impl InstanceCollector {
    pub fn visit_type_pack_id_type_function_instance_type_pack(
        &mut self,
        tp: TypePackId,
        tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        self.type_function_instance_stack.push(tp as *const c_void);

        let guess_depth = luaur_common::DFInt::LuauTypeFamilyUseGuesserDepth.get();
        if guess_depth >= 0 && self.type_function_instance_stack.len() as i32 > guess_depth {
            self.should_guess.insert(tp as *const c_void);
        }

        if !self.recorded_tps.contains(&tp) {
            self.recorded_tps.insert(tp);
            self.tps.push_front(tp);
        }

        for &p in &tfitp.typeArguments {
            self.traverse_type_id(p);
        }

        for &p in &tfitp.packArguments {
            self.traverse_type_pack_id(p);
        }

        self.type_function_instance_stack.pop();

        false
    }
}
