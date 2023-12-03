const DEBUG: bool = false;

macro_rules! dprintln {
    ($($arg:expr),+) => {
        if DEBUG {
            eprintln!($($arg),*);
        }
    };
}

pub(crate) use dprintln;
