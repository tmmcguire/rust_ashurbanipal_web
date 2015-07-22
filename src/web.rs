//! Web interface routines.

/*
 * ashurbanipal.web: Java Servlet-based interface to Ashurbanipal data
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

use std::error::Error;
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
        let style = Style::read(style_path);
        let topic = Topic::read(topic_path);
        let metadata = Metadata::read(metadata_path);
        let index    = Index::new(&metadata);
        RecState( style, topic, metadata, index )
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
        let &RecState(ref style, ref topic, _, _)
            = panic_unless!("recstate", option: context.global.get());
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
    let &RecState(_, _, ref metadata, _)
        = panic_unless!("recstate", option: context.global.get());
    let start = optional("start", 0, &context);
    let limit = optional("limit", 20, &context);
    match required("etext_no", &context) {
        Some(etext_no) => {
            match r.sorted_results(etext_no) {
                Some(rows) => {
                    let recommendation = Recommendations {
                        count : rows.len(),
                        rows  : metadata.add_metadata(&rows, start, limit)
                    };
                    match json::encode(&recommendation) {
                        Ok(json) => {
                            response.set_status(StatusCode::Ok);
                            response.into_writer().send(json);
                        }
                        Err(e) => {
                            response.set_status(StatusCode::InternalServerError);
                            response.into_writer().send(e.description());
                        }
                    }
                }
                None => {
                    response.set_status(StatusCode::NotFound);
                    response.into_writer().send("no matching etext");
                }
            }
        }
        None => {
            response.set_status(StatusCode::BadRequest);
            response.into_writer().send("parameter required: etext_no");
        }
    };
}

fn handle_text_query(context: Context, mut response: Response) {
    let &RecState(_, _, ref metadata, _)
        = panic_unless!("recstate", option: context.global.get());
    match required_path("etext_no", &context) {
        Some(etext_no) => {
            match metadata.get(etext_no) {
                Some(text) => {
                    match json::encode(text) {
                        Ok(json) => {
                            response.set_status(StatusCode::Ok);
                            response.into_writer().send(json);
                        }
                        Err(e) => {
                            response.set_status(StatusCode::InternalServerError);
                            response.into_writer().send(e.description());
                        }
                    }
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
    let &RecState(_, _, ref metadata, ref index)
        = panic_unless!("recstate", option: context.global.get());
    let start = optional("start", 0, &context);
    let limit = optional("limit", 20, &context);
    match required::<String>("query", &context) {
        Some(query) => {
            let rows = index.get_entries(&query);
            let recommendations = Recommendations {
                count : rows.len(),
                rows  : metadata.add_metadata(&rows, start, limit),
            };
            match json::encode(&recommendations) {
                Ok(json) => {
                    response.set_status(StatusCode::Ok);
                    response.into_writer().send(json);
                }
                Err(e) => {
                    response.set_status(StatusCode::InternalServerError);
                    response.into_writer().send(e.description());
                }
            }
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

fn optional<T:FromStr>(v : &str, default : T, context : &Context) -> T {
    required(v, context).unwrap_or(default)
}

fn required_path<T:FromStr>(v: &str, context: &Context) -> Option<T> {
    context.variables.get(v).and_then( |s| s.parse().ok() )
}

#[derive(RustcEncodable)]
struct Recommendations<'a> {
    count : usize,
    rows  : Vec<TextRef<'a>>,
}
