//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:159:set_iteration`
//! Source: `Analysis/include/Luau/Set.h:159` (hand-ported placeholder)
//!
//! `Luau::Set` iteration (begin/end/const_iterator) is not yet ported — no
//! current caller iterates a Set. Port from Set.h:104-199 when one does;
//! the underlying `DenseHashMap::iter` filtered on `present == true` is the
//! direct shape.
