//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1391:type_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:1391-1395` (hand-ported)

use crate::records::function_argument::FunctionArgument;
use crate::records::type_pack_stringifier::TypePackStringifier;
use crate::records::type_stringifier::TypeStringifier;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl TypeStringifier {
    /// C++ `void TypeStringifier::stringify(TypePackId tpid, const std::vector<std::optional<FunctionArgument>>& names)`.
    pub fn stringify_type_pack_id_vector_optional_function_argument(
        &mut self,
        tpid: TypePackId,
        names: &Vec<Option<FunctionArgument>>,
    ) {
        let mut tps = TypePackStringifier::type_pack_stringifier_stringifier_state_vector_optional_function_argument(
            self.state, names,
        );
        tps.stringify_type_pack_id(tpid);
    }
}
