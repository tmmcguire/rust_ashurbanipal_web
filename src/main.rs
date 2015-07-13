#[macro_use]
extern crate rustful;
extern crate rustc_serialize;
extern crate iterator_utilities;

mod combination;
mod index;
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
            "style"            => Get : RecQuery::Style,
            "topic"            => Get : RecQuery::Topic,
            "combination"      => Get : RecQuery::Combination,
            "lookup/:etext_no" => Get : RecQuery::TextLookup,
            "lookup"           => Get : RecQuery::TextSearch
        }
    };

    let server = Server {
        content_type : content_type!(Application / Json; Charset = Utf8),
        global       : (RecState::new(&args[1], &args[2], &args[3]),).into(),
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
