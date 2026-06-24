use crate::records::function_type::FunctionType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeFunctionReductionGuesser {
    pub fn is_function_generics_saturated(
        &self,
        ftv: &FunctionType,
        args_used: &mut DenseHashSet<TypeId>,
    ) -> bool {
        let same_size = ftv.generics.len() == args_used.size();
        let mut all_generics_appear = true;
        for &gt in &ftv.generics {
            all_generics_appear = all_generics_appear && args_used.contains(&gt);
        }
        same_size && all_generics_appear
    }
}
