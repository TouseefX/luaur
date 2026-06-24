#[macro_export]
macro_rules! CHECK_LONG_STRINGS_EQ {
    ($a:expr, $b:expr) => {
        let aa = $a;
        let bb = $b;
        let a_lines = luaur_common::functions::split::split(&aa, '\n');
        let b_lines = luaur_common::functions::split::split(&bb, '\n');

        assert_eq!(
            a_lines.len(),
            b_lines.len(),
            "Line counts don't match: {} != {}",
            a_lines.len(),
            b_lines.len()
        );

        let min_len = core::cmp::min(a_lines.len(), b_lines.len());

        for i in 0..min_len {
            let a_line = luaur_common::functions::strip::strip(a_lines[i]);
            let b_line = luaur_common::functions::strip::strip(b_lines[i]);

            assert_eq!(
                a_line, b_line,
                "Mismatch on line {} between:\n\t«{}»\nand\t«{}»\n",
                i, a_line, b_line
            );
        }
    };
}

pub use CHECK_LONG_STRINGS_EQ;
