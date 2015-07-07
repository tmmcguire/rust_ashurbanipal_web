//! Web interface routines.

use std::path::Path;

use rustful::{Context,Handler,Response,StatusCode};

use rustc_serialize::json;

use recommendation::{Etext,Recommendation};
use combination::Combination;
use style::Style;
use topic::Topic;

pub struct RecState(Style, Topic);

impl RecState {
    pub fn new<P : AsRef<Path>>(style_path:P, topic_path:P) -> RecState {
            RecState( Style::read(style_path), Topic::read(topic_path) )
    }
}

pub enum RecQuery {
    Style,
    Topic,
    Combination,
}

impl Handler for RecQuery {
    fn handle_request(&self, context: Context, response: Response) {
        let &RecState(ref style, ref topic) = context.global.get::<RecState>().unwrap();
        match *self {
            RecQuery::Style       => handle(style, context, response),
            RecQuery::Topic       => handle(topic, context, response),
            RecQuery::Combination => handle(&Combination::new(style, topic), context, response)
        }
    }
}

fn handle(r : &Recommendation, context: Context, mut response: Response) {
    let start = optional("start", 0, &context);
    let limit = optional("limit", 20, &context);
    match required("etext_no", &context) {
        Some(etext_no) => {
            let rows = r.sorted_results(etext_no).unwrap();
            response.set_status(StatusCode::Ok);
            response.into_writer().send( json::encode( &Recommendations {
                count : rows.len(),
                rows  : rows.iter()
                    .skip(start).take(limit)
                    .map( |&(e,s)| Text { etext_no : e, score : s } )
                    .collect()
            } ).unwrap() );
        }
        None => {
            response.set_status(StatusCode::BadRequest);
            response.into_writer().send("parameter required: etext_no");
        }
    };
}

fn required(v : &str, context : &Context) -> Option<usize> {
    context.query.get(v).and_then( |s| s.parse::<usize>().ok() )
}

fn optional(v : &str, default : usize, context : &Context) -> usize {
    required(v, context).unwrap_or(default)
}

#[derive(RustcEncodable)]
struct Text {
    etext_no : Etext,
    score    : f64,
}

#[derive(RustcEncodable)]
struct Recommendations {
    count : usize,
    rows  : Vec<Text>,
}
