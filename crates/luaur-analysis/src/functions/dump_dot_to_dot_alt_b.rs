use crate::functions::to_dot_to_dot_alt_b::to_dot_type_pack_id_to_dot_options;
use crate::records::to_dot_options::ToDotOptions;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn dump_dot(tp: TypePackId) {
    let opts = ToDotOptions {
        show_pointers: true,
        duplicate_primitives: true,
    };

    let dot = to_dot_type_pack_id_to_dot_options(tp, &opts);
    println!("{}", dot);
}
