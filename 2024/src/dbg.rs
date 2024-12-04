#[allow(unused)]
pub const DEBUG: bool = false;

#[allow(unused)]
macro_rules! dprintln {
    ($($arg:expr),+) => {
        if crate::dbg::DEBUG {
            eprintln!($($arg),*);
        }
    };
}

#[allow(unused)]
pub(crate) use dprintln;
