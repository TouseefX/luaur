use crate::functions::read::read;

pub unsafe fn read_var_int(data: *const u8, offset: &mut usize) -> u32 {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;
    let mut byte: u8;

    loop {
        byte = read::<u8>(data, offset);
        result |= ((byte & 127) as u32) << shift;
        shift += 7;
        if (byte & 128) == 0 {
            break;
        }
    }

    result
}
