//! Macros used by ashurbanipal_web.

/*
 * ashurbanipal.web: Rust Rustful-based interface to Ashurbanipal data
 * Copyright 2015 Tommy M. McGuire
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or (at
 * your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301 USA.
 */

#[macro_export]
macro_rules! panic_unless {
    ($m:expr,option: $e:expr) => ( match $e { Some(v) => v,
                                              None => panic!($m),
    } );
    ($m:expr,result: $e:expr) => ( match $e { Ok(v) => v,
                                              Err(e) => panic!(format!("{}: {}", $m, e)),
    } )
}
