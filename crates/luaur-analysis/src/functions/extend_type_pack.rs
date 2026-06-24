use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::track_interior_free_type::track_interior_free_type;
use crate::functions::track_interior_free_type_pack::track_interior_free_type_pack;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;

pub fn extend_type_pack(
    arena: &mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    pack: TypePackId,
    length: usize,
    overrides: Vec<Option<TypeId>>,
) -> TypePack {
    let mut result = TypePack {
        head: alloc::vec::Vec::new(),
        tail: None,
    };

    let mut current_pack = pack;

    loop {
        current_pack = unsafe { follow_type_pack_id(current_pack) };

        let p = unsafe { get_type_pack_id::<TypePack>(current_pack) };
        if !p.is_null() {
            let p = unsafe { &*p };
            let mut i = 0;
            while i < p.head.len() && result.head.len() < length {
                result.head.push(p.head[i]);
                i += 1;
            }

            if result.head.len() == length {
                if i == p.head.len() {
                    result.tail = p.tail;
                } else {
                    let new_tail = arena.add_type_pack_t(TypePack {
                        head: p.head[i..].to_vec(),
                        tail: p.tail,
                    });
                    result.tail = Some(new_tail);
                }
                return result;
            } else if let Some(tail) = p.tail {
                current_pack = tail;
                continue;
            } else {
                return result;
            }
        } else {
            let vtp = unsafe { get_type_pack_id::<VariadicTypePack>(current_pack) };
            if !vtp.is_null() {
                let vtp = unsafe { &*vtp };
                while result.head.len() < length {
                    result.head.push(vtp.ty);
                }
                result.tail = Some(current_pack);
                return result;
            }

            let ftp = unsafe { get_mutable_type_pack_id::<FreeTypePack>(current_pack) };
            if !ftp.is_null() {
                let ftp = unsafe { &mut *ftp };
                let new_pack_scope = ftp.scope;
                let new_pack_polarity = ftp.polarity;

                let mut new_pack = TypePack {
                    head: alloc::vec::Vec::new(),
                    tail: Some(arena.fresh_type_pack(new_pack_scope, new_pack_polarity)),
                };

                track_interior_free_type_pack(new_pack_scope, new_pack.tail.unwrap());

                result.tail = new_pack.tail;

                let mut overrides_index = 0;
                while result.head.len() < length {
                    let t = if overrides_index < overrides.len()
                        && overrides[overrides_index].is_some()
                    {
                        overrides[overrides_index].unwrap()
                    } else {
                        let ft = FreeType {
                            index: 0,
                            level: TypeLevel::default(),
                            scope: new_pack_scope,
                            forwarded_type_alias: false,
                            lower_bound: unsafe { (*builtin_types).neverType },
                            upper_bound: unsafe { (*builtin_types).unknownType },
                            polarity: new_pack_polarity,
                        };
                        let new_ty = arena.add_type(ft);
                        track_interior_free_type(new_pack_scope, new_ty);
                        new_ty
                    };

                    new_pack.head.push(t);
                    result.head.push(*new_pack.head.last().unwrap());
                    overrides_index += 1;
                }

                unsafe {
                    (*as_mutable_type_pack_id(current_pack)).ty =
                        TypePackVariant::TypePack(new_pack);
                }

                return result;
            }

            if !unsafe { get_type_pack_id::<ErrorTypePack>(current_pack) }.is_null() {
                while result.head.len() < length {
                    result.head.push(unsafe { (*builtin_types).errorType });
                }
                result.tail = Some(current_pack);
                return result;
            } else {
                result.tail = Some(current_pack);
                return result;
            }
        }
    }
}
