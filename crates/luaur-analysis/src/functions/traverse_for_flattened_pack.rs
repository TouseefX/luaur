//! Source: `Analysis/src/TypePath.cpp:1143-1180` (hand-ported)
use crate::enums::pack_field::PackField;
use crate::functions::flatten_pack_with_path::flatten_pack_with_path;
use crate::functions::traverse_for_pack_type_path::traverse_for_pack;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::path::Path;
use crate::records::type_arena::TypeArena;
use crate::records::type_pack::TypePack;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn traverse_for_flattened_pack(
    root: TypeId,
    path: &Path,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
) -> TypePack {
    // Iterate over path's components, and figure out when it turns into Tails and
    // GenericPackMappings. We want to split out the part of the path that
    // contains the generic pack mappings we're interested in, so that we can
    // flatten it. path[splitIndex:] will contain only Tails and
    // GenericPackMappings.
    let mut split_index: usize = 0;
    let mut i = path.components.len();
    while i > 0 {
        let c = &path.components[i - 1];

        let is_not_tail = !matches!(c, Component::PackField(pf) if *pf == PackField::Tail);
        let is_not_gpm = !matches!(c, Component::GenericPackMapping(_));

        if is_not_tail && is_not_gpm {
            split_index = i;
            break;
        }
        i -= 1;
    }

    // Root is a TypeId, not a TypePackId, so splitIndex should be > 0
    LUAU_ASSERT!(split_index > 0);
    if split_index == path.components.len() || split_index == 0 {
        return TypePack {
            head: alloc::vec::Vec::new(),
            tail: None,
        };
    }

    let base_path = Path::from_components(path.components[..split_index].to_vec());
    let base_pack = traverse_for_pack(root, &base_path, builtin_types, arena);

    let base_pack = match base_pack {
        Some(bp) => bp,
        None => {
            LUAU_ASSERT!(false /* "Expected to be able to traverse to a TypePackId" */);
            return TypePack {
                head: alloc::vec::Vec::new(),
                tail: None,
            };
        }
    };

    let suffix_path = Path::from_components(path.components[split_index..].to_vec());
    flatten_pack_with_path(base_pack, &suffix_path)
}
