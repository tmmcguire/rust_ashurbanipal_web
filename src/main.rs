
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

#[macro_use]
extern crate rustful;
extern crate rustc_serialize;
extern crate iterator_utilities;

#[macro_use]
mod macros;

mod combination;
mod index;
mod matrix;
mod mbitset;
mod metadata;
mod nysiis;
mod recommendation;
mod style;
mod topic;
mod web;

use std::env;
use std::error::Error;
use std::str::FromStr;

use rustful::{Server,TreeRouter};

use web::{RecQuery,RecState};

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 4 { panic!("Usage: ashurbanipal_web pos-data topic-data metadata"); }

    let router = insert_routes! {
        TreeRouter::new() => {
            "ashurbanipal.web/data/file" => {
                "style" => Get: RecQuery::Style,
                "topic" => Get: RecQuery::Topic,
                "combination" => Get: RecQuery::Combination,
                "lookup" => {
                    Get: RecQuery::TextSearch,
                    ":etext_no" => Get: RecQuery::TextLookup,
                }
            }
        }
    };

    let rec_state = RecState::new(&args[1], &args[2], &args[3]);

    println!("serving...");

    let server = Server {
        content_type : content_type!(Application / Json; Charset = Utf8),
        global       : (rec_state,).into(),
        handlers     : router,
        host         : FromStr::from_str("127.0.0.1:8080").unwrap(),
        log          : Box::new( rustful::log::StdOut ),
        server       : "ashurbanipal_web(Rust)".to_string(),
        ..Server::default()
    };

    if let Err(e) = server.run() {
        println!("could not start server: {}", e.description());
    }
}
