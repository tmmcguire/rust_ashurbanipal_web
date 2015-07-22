
#[macro_export]
macro_rules! panic_unless {
    ($m:expr,option: $e:expr) => ( match $e { Some(v) => v,
                                              None => panic!($m),
    } );
    ($m:expr,result: $e:expr) => ( match $e { Ok(v) => v,
                                              Err(e) => panic!(format!("{}: {}", $m, e)),
    } )
}
