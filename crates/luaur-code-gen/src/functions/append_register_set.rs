use crate::records::ir_to_string_context::IrToStringContext;
use crate::records::register_set::RegisterSet;
use core::ffi::c_char;
use core::ffi::CStr;

pub fn append_register_set(
    ctx: &mut IrToStringContext,
    rs: &RegisterSet,
    separator: *const c_char,
) {
    let mut comma = false;
    let sep_str = unsafe { CStr::from_ptr(separator).to_string_lossy() };

    for i in 0..256 {
        let word_idx = i / 64;
        let bit_idx = i % 64;

        if (rs.regs[word_idx] & (1 << bit_idx)) != 0 {
            if comma {
                ctx.result.push_str(&sep_str);
            }
            comma = true;

            use core::fmt::Write;
            let _ = write!(ctx.result, "R{}", i);
        }
    }

    if rs.vararg_seq {
        if comma {
            ctx.result.push_str(&sep_str);
        }

        use core::fmt::Write;
        let _ = write!(ctx.result, "R{}...", rs.vararg_start);
    }
}
