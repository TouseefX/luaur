use crate::enums::relation::Relation;

pub fn invert_relation(r: Relation) -> Relation {
    match r {
        Relation::Disjoint => Relation::Subset,
        Relation::Coincident => Relation::Disjoint,
        Relation::Intersects => Relation::Intersects,
        Relation::Subset => Relation::Disjoint,
        Relation::Superset => Relation::Intersects,
    }
}
