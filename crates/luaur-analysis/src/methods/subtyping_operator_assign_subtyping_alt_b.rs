use crate::records::subtyping::Subtyping;

impl Subtyping {
    #[allow(non_snake_case)]
    pub fn operator_assign_mut(&mut self, other: Subtyping) {
        self.builtin_types = other.builtin_types;
        self.arena = other.arena;
        self.normalizer = other.normalizer;
        self.type_function_runtime = other.type_function_runtime;
        self.ice_reporter = other.ice_reporter;
        self.limits = other.limits;
        self.unique_types = other.unique_types;
        self.seen_types = other.seen_types;
        self.seen_packs = other.seen_packs;
        self.result_cache = other.result_cache;
    }
}
