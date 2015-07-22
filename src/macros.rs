
#[macro_export]
macro_rules! unpack {
    ($e:expr,$m:expr) => ( match $e {
        Ok(v) => v,
        Err(e) => panic!(format!("{}: {}", $m, e.to_string())),
        })
}

/// Macro for handling Options
#[macro_export]
macro_rules! expect {
    ($e:expr,$m:expr) => ( match $e { Some(e) => e, None => panic!($m), })
}
