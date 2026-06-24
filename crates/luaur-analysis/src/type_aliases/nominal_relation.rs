use crate::records::klass::Klass;
use crate::records::obj::Obj;
use luaur_common::records::variant::Variant2;

pub type NominalRelation = Variant2<Obj, Klass>;
