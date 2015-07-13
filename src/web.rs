//! Web interface routines.

use std::path::Path;
use std::str::FromStr;

use rustful::{Context,Handler,Response,StatusCode};

use rustc_serialize::json;

use combination::Combination;
use index::Index;
use metadata::{TextRef,Metadata};
use recommendation::Recommendation;
use style::Style;
use topic::Topic;

pub struct RecState(Style, Topic, Metadata, Index);

impl RecState {
    pub fn new<P : AsRef<Path>>(style_path:P, topic_path:P, metadata_path:P) -> RecState {
        let metadata = Metadata::read(metadata_path);
        let index    = Index::new(&metadata);
        RecState(
            Style::read(style_path),
            Topic::read(topic_path),
            metadata,
            index,
            )
    }
}

pub enum RecQuery {
    Style,
    Topic,
    Combination,
    TextLookup,
    TextSearch,
}

impl Handler for RecQuery {
    fn handle_request(&self, context: Context, response: Response) {
        let &RecState(ref style, ref topic, _, _) = context.global.get().unwrap();
        match *self {
            RecQuery::Style       => handle_recommendation_query(style, context, response),
            RecQuery::Topic       => handle_recommendation_query(topic, context, response),
            RecQuery::Combination => handle_recommendation_query(&Combination::new(style, topic), context, response),
            RecQuery::TextLookup  => handle_text_query(context, response),
            RecQuery::TextSearch  => handle_text_search(context, response),
        }
    }
}
    
fn handle_recommendation_query(r : &Recommendation, context: Context, mut response: Response) {
    let &RecState(_, _, ref metadata, _) = context.global.get().unwrap();
    let start = optional("start", 0, &context);
    let limit = optional("limit", 20, &context);
    match required("etext_no", &context) {
        Some(etext_no) => {
            let rows = r.sorted_results(etext_no).unwrap();
            response.set_status(StatusCode::Ok);
            response.into_writer().send( json::encode(
                &Recommendations {
                    count : rows.len(),
                    rows  : rows.iter()
                        .skip(start).take(limit)
                        .map( |&(e,s)| (metadata.get(e),s) )
                        .filter( |&(ref o,_)| o.is_some() )
                        .map( |(ref o,s)| o.unwrap().score(s) )
                        .collect()
                } ).unwrap() );
        }
        None => {
            response.set_status(StatusCode::BadRequest);
            response.into_writer().send("parameter required: etext_no");
        }
    };
}

fn handle_text_query(context: Context, mut response: Response) {
    let &RecState(_, _, ref metadata, _) = context.global.get::<RecState>().unwrap();
    match required_path("etext_no", &context) {
        Some(etext_no) => {
            match metadata.get(etext_no) {
                Some(text) => {
                    response.set_status(StatusCode::Ok);
                    response.into_writer().send( json::encode( text ).unwrap() )
                }
                None => {
                    response.set_status(StatusCode::NotFound);
                    response.into_writer().send(format!("no matching etext: {}", etext_no));
                }
            }
        }
        None => {
            response.set_status(StatusCode::BadRequest);
            response.into_writer().send("parameter problem: lookup/<etext_no>");
        }
    }
}

fn handle_text_search(context: Context, mut response: Response) {
    let &RecState(_, _, ref metadata, ref index) = context.global.get().unwrap();
    let start = optional("start", 0, &context);
    let limit = optional("limit", 20, &context);
    match required::<String>("query", &context) {
        Some(query) => {
            let rows = index.get_entries(&query);
            response.set_status(StatusCode::Ok);
            response.into_writer().send( json::encode(
                &Recommendations {
                    count : rows.len(),
                    rows  : rows.iter()
                        .skip(start).take(limit)
                        .map( |&(e,s)| (metadata.get(e),s) )
                        .filter( |&(ref o,_)| o.is_some() )
                        .map( |(ref o,s)| o.unwrap().score(s) )
                        .collect()
                } ).unwrap() );
        }
        None => {
            response.set_status(StatusCode::BadRequest);
            response.into_writer().send("missing argument: query");
        }
    }
}

fn required<T:FromStr>(v : &str, context : &Context) -> Option<T> {
    context.query.get(v).and_then( |s| s.parse::<T>().ok() )
}

fn optional(v : &str, default : usize, context : &Context) -> usize {
    required(v, context).unwrap_or(default)
}

fn required_path(v: &str, context: &Context) -> Option<usize> {
    context.variables.get(v).and_then( |s| s.parse::<usize>().ok() )
}

#[derive(RustcEncodable)]
struct Recommendations<'a> {
    count : usize,
    rows  : Vec<TextRef<'a>>,
}
