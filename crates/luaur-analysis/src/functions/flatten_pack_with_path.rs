//! Source: `Analysis/src/TypePath.cpp:1107-1141` (hand-ported)
use crate::enums::pack_field::PackField;
use crate::functions::end_type_pack::end;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::path::Path;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn flatten_pack_with_path(root: TypePackId, path: &Path) -> TypePack {
    let mut flattened: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();

    let mut curr: Option<TypePackId> = Some(root);
    let mut path_iter: usize = 0;
    let path_end = path.components.len();

    while let Some(curr_pack) = curr {
        let mut it = TypePackIterator::type_pack_iterator();
        it.type_pack_iterator_type_pack_id(curr_pack);

        // Push back curr's head
        while it.operator_ne(&end(curr_pack)) {
            flattened.push(*it.operator_deref());
            it.operator_inc();
        }

        // Check if curr has a tail, and if the next bit of path is Tail +
        // GenericPackMapping
        curr = it.tail();
        let has_generic_tail = match curr {
            Some(tail) => !unsafe { get_type_pack_id::<GenericTypePack>(tail) }.is_null(),
            None => false,
        };
        if !has_generic_tail || path_iter == path_end {
            break;
        }

        // const TypePath::PackField* pf = get_if<PackField>(&*pathIter);
        // if (!pf || *pf != Tail) break;
        match path.components.get(path_iter) {
            Some(Component::PackField(pf)) if *pf == PackField::Tail => {}
            _ => break,
        }

        path_iter += 1;

        // const GenericPackMapping* gpm = get_if<GenericPackMapping>(&*pathIter);
        // if (!gpm) break;
        let gpm = match path.components.get(path_iter) {
            Some(Component::GenericPackMapping(gpm)) => *gpm,
            _ => break,
        };

        path_iter += 1;
        curr = Some(gpm.mappedType);
    }

    TypePack {
        head: flattened,
        tail: curr,
    }
}
