pub const DEBUG: bool = false;

macro_rules! dprintln {
    ($($arg:expr),+) => {
        if crate::dbg::DEBUG {
            eprintln!($($arg),*);
        }
    };
}

pub(crate) use dprintln;
