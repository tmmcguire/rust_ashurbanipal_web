//! Index for finding Project Gutenberg texts by subject, author, or title.

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

use std::cmp::{Ord,Ordering};
use std::collections::HashMap;

use iterator_utilities::equivalence_class::equivalence_classes;

use metadata::Metadata;
use nysiis::encode;
use recommendation::{Etext,Score};

type ScoredResult = (Etext,Score);

type ScoredDictionary = HashMap<String,Vec<ScoredResult>>;

#[derive(Debug)]
pub struct Index {
    index: ScoredDictionary,
}

impl Index {
    pub fn new(metadata: &Metadata) -> Index {
        // Compute a vector of keyword, etext_no, simple score triples.
        let mut postings: Vec<(String,Etext,Score)> = Vec::new();
        for (&etext_no, text) in metadata.iter() {
            postings.extend( text.title.split(' ').map(encode).map( |t| (t, etext_no, 3.0) ) );
            postings.extend( text.author.split(' ').map(encode).map( |a| (a, etext_no, 2.0) ) );
            postings.extend( text.subject.split(' ').map(encode).map( |s| (s, etext_no, 1.0) ) );
        }
        // Sort postings by keyword, then by etext_no.
        postings.sort_by(compare);
        // Accumulate scores for keyword, etext_no, then insert
        // etext_no and combined score into index under keyword.
        let mut index = HashMap::new();
        for cls in equivalence_classes(&postings, |l,r| l.0 == r.0 && l.1 == r.1 ) {
            let r: (&str,Etext,Score) =
                cls.fold(("", 0, 0 as Score), |a,p| (&p.0, p.1, a.2+p.2));
            index.entry(r.0.to_string()).or_insert( Vec::new() ).push( (r.1,r.2) );
        }
        // Sort stored postings lists by etext_no.
        for (_,postings) in index.iter_mut() {
            postings.sort_by(|l,r| l.0.cmp(&r.0));
        }
        Index { index: index }
    }

    pub fn get_entries(&self, s: &str) -> Vec<ScoredResult> {
        let mut results = Vec::new();
        for key in s.split(' ').map( encode ) {
            self.accept_or_merge_postings(&mut results, &key);
        }
        // Sort results by score, decreasing.
        results.sort_by( |l,r| {
            match l.1.partial_cmp(&r.1) {
                Some(o) => o.reverse(),
                // floating point numbers are a pain in the ass.
                None    => unimplemented!()
            }
        });
        results
    }

    fn accept_or_merge_postings(&self, results: &mut Vec<ScoredResult>, key: &String) {
        match self.index.get(key) {
            Some(postings) => {
                if results.len() == 0 {
                    for posting in postings.iter() {
                        results.push(posting.clone());
                    }
                } else {
                    merge_postings(results, postings);
                }
            }
            None => { }
        }
    }
}

fn merge_postings(results: &mut Vec<ScoredResult>, postings: &Vec<ScoredResult>) {
    let mut r = 0;
    let mut p = 0;
    while r < results.len() && p < postings.len() {
        if results[r].0 > postings[p].0 {
            p += 1;
        } else if results[r].0 < postings[p].0 {
            results.remove(r);
        } else /* results[r].0 == postings[p].0 */ {
            results[r].1 += postings[p].1;
            r += 1;
            p += 1;
        }
    }
    while r < results.len() {
        results.remove(r);
    }
}

fn compare(left: &(String,Etext,Score), right: &(String,Etext,Score)) -> Ordering {
    match left.0.cmp(&right.0) {
        Ordering::Equal => left.1.cmp(&right.1),
        other           => other
    }
}
