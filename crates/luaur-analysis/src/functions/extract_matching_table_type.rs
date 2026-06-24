use crate::enums::relation::Relation;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type::FreeType;
use crate::records::table_type::TableType;
use crate::records::type_ids::TypeIds;
use crate::records::type_iterator::TypeIterator;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn extract_matching_table_type(
    expected_union: &UnionType,
    expr_type: TypeId,
    _builtin_types: *mut BuiltinTypes,
) -> Option<TypeId> {
    unsafe {
        LUAU_ASSERT!(luaur_common::FFlag::LuauBidirectionalInferenceBetterUnionHandling.get());
        let expr_table = get_type_id::<TableType>(follow_type_id(expr_type));
        if expr_table.is_null() {
            return None;
        }
        let expr_table = &*expr_table;

        // Try to filter out tables based on property names, for example
        // if we are considering the type ...
        //
        //  { foo: number, bar: string } | { foo: number, baz: boolean }
        //
        // ... and the table in question looks like ...
        //
        //  { baz = true }
        //
        // ... the user probably intends the second definition.
        let mut potential_tables = TypeIds::type_ids();

        // C++ iterates `for (TypeId ty : expectedUnion)`, which uses
        // `UnionTypeIterator`: each yielded `ty` is already `follow`-ed and
        // nested unions are flattened (`TypeIterator::operator*`/`descend`).
        // Iterating `expected_union.options` directly would hand un-followed
        // bound types to `get<TableType>`, tripping its no-bound-type assert.
        let mut it =
            TypeIterator::<UnionType>::type_iterator_type(expected_union as *const UnionType);
        let end_it = TypeIterator::<UnionType>::type_iterator_default();
        while it.operator_ne(&end_it) {
            let ty = it.operator_deref();
            it.operator_inc();

            let tt = get_type_id::<TableType>(ty);
            if !tt.is_null() {
                let mut is_disjoint = false;
                // NOTE: We iterate over the expected properties for structural subtyping reasons,
                // consider:
                //
                //  local t: { foo: number? } = {
                //      foo = 42,
                //      -- 10,000 properties not shown.
                //  }
                //
                // Those 10k properties do not matter here.
                for (name, expected_prop) in &(*tt).props {
                    // If the property from the expected type is not in the
                    // expression, skip it.
                    let prop_in_table_expr = match expr_table.props.get(name) {
                        Some(p) => p,
                        None => continue,
                    };

                    // Also, if the expected type does not have a read component, skip this.
                    let expected_read_ty = match expected_prop.read_ty {
                        Some(t) => t,
                        None => continue,
                    };

                    let expr_prop = prop_in_table_expr;

                    // If the expression property doesn't have a read type, then
                    // we cannot reasonably check this against the read type of
                    // the expected property.
                    let expr_read_ty = match expr_prop.read_ty {
                        Some(t) => t,
                        None => {
                            // Also assert here: we should never encounter an inferred
                            // write-only type from an expression.
                            LUAU_ASSERT!(
                                false /* "Unexpected write-only property inside table literal." */
                            );
                            continue;
                        }
                    };

                    let expected_prop_type = follow_type_id(expected_read_ty);
                    let expr_prop_type = follow_type_id(expr_read_ty);

                    if relate_type_id_type_id(expected_prop_type, expr_prop_type)
                        == Relation::Disjoint
                    {
                        is_disjoint = true;
                        break;
                    }

                    let ft = get_type_id::<FreeType>(expr_prop_type);
                    if !ft.is_null()
                        && relate_type_id_type_id((*ft).lower_bound, expected_prop_type)
                            == Relation::Disjoint
                    {
                        is_disjoint = true;
                        break;
                    }
                }

                if !is_disjoint {
                    potential_tables.insert_type_id(ty);
                }
            }
        }

        if potential_tables.size() == 1 {
            return Some(potential_tables.order[0]);
        }

        None
    }
}
