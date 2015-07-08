#[macro_use]
extern crate rustful;
extern crate rustc_serialize;

mod combination;
mod mbitset;
mod metadata;
mod recommendation;
mod style;
mod topic;
mod web;

use std::env;
use std::error::Error;

use rustful::{Server,TreeRouter};
use rustful::Method::Get;

use web::{RecQuery,RecState};

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 4 { panic!("Usage: ashurbanipal_web pos-data topic-data metadata"); }

    let router = insert_routes! {
        TreeRouter::new() => {
            "style"            => Get : RecQuery::Style,
            "topic"            => Get : RecQuery::Topic,
            "combination"      => Get : RecQuery::Combination,
            "lookup/:etext_no" => Get : RecQuery::TextLookup
        }
    };

    let server_result = Server {
        host         : 8080.into(),
        handlers     : router,
        content_type : content_type!(Application / Json; Charset = Utf8),
        global       : (RecState::new(&args[1], &args[2], &args[3]),).into(),
        ..Server::default()
    }.run();

    match server_result {
        Ok(_) => {},
        Err(e) => println!("could not start server: {}", e.description()),
    }
}
