use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::block_iterator_wrapper::BlockIteratorWrapper;
use crate::records::cfg_info::CfgInfo;

pub fn successors(cfg: &CfgInfo, blockIdx: u32) -> BlockIteratorWrapper {
    // Keep behavior consistent with the C++ CODEGEN_ASSERT without triggering the
    // current macro's type mismatch for luaur_common::assert_call_handler.
    assert!(blockIdx < cfg.successors_offsets.len() as u32);

    let start = cfg.successors_offsets[blockIdx as usize];
    let end = if blockIdx + 1 < cfg.successors_offsets.len() as u32 {
        cfg.successors_offsets[(blockIdx + 1) as usize]
    } else {
        cfg.successors.len() as u32
    };

    BlockIteratorWrapper {
        itBegin: unsafe { cfg.successors.as_ptr().add(start as usize) },
        itEnd: unsafe { cfg.successors.as_ptr().add(end as usize) },
    }
}
