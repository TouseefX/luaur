use crate::enums::polarity::Polarity;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::quantifier::Quantifier;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

impl Quantifier {
    pub fn visit_type_pack_id_free_type_pack(
        &mut self,
        tp: TypePackId,
        ftp: &FreeTypePack,
    ) -> bool {
        self.seen_mutable_type = true;

        if !self.level.subsumes(&ftp.level) {
            return false;
        }

        // *asMutable(tp) = GenericTypePack{level};
        let mut gtp = GenericTypePack {
            index: 0,
            level: Default::default(),
            scope: core::ptr::null_mut(),
            name: Default::default(),
            explicitName: false,
            polarity: Polarity::None,
        };
        gtp.generic_type_pack_type_level(self.level);

        unsafe {
            *as_mutable_type_pack_id(tp) = TypePackVar::from(gtp);
        }

        self.generic_packs.push(tp);
        true
    }
}
