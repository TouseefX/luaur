use crate::records::block_ordering::BlockOrdering;

pub fn find_common_dominator(
    idoms: &Vec<u32>,
    data: &Vec<BlockOrdering>,
    mut a: u32,
    mut b: u32,
) -> u32 {
    while a != b {
        while data[a as usize].postOrder < data[b as usize].postOrder {
            a = idoms[a as usize];
            if a == !0u32 {
                // Keep behavior native-only assertion without relying on CODEGEN_ASSERT.
                panic!("CODEGEN_ASSERT failed: a != !0u32");
            }
        }

        while data[b as usize].postOrder < data[a as usize].postOrder {
            b = idoms[b as usize];
            if b == !0u32 {
                // Keep behavior native-only assertion without relying on CODEGEN_ASSERT.
                panic!("CODEGEN_ASSERT failed: b != !0u32");
            }
        }
    }

    a
}
