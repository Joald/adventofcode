pub const DEBUG: bool = true;

macro_rules! dprintln {
    ($($arg:expr),+) => {
        if crate::dbg::DEBUG {
            eprintln!($($arg),*);
        }
    };
}

pub(crate) use dprintln;
