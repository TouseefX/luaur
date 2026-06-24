use crate::functions::to_dot_to_dot::to_dot_type_id_to_dot_options;
use crate::records::to_dot_options::ToDotOptions;
use crate::type_aliases::type_id::TypeId;

pub fn dump_dot(ty: TypeId) {
    let opts = ToDotOptions {
        show_pointers: true,
        duplicate_primitives: true,
    };

    let dot = to_dot_type_id_to_dot_options(ty, &opts);
    println!("{}", dot);
}
