//! Index for finding Project Gutenberg texts by subject, author, or title.

use std::cmp::{Ord,Ordering};
use std::collections::HashMap;

use metadata::Metadata;
use nysiis::encode;
use recommendation::Etext;

type ScoredResult = (Etext,f64);

type ScoredDictionary = HashMap<String,Vec<ScoredResult>>;

#[derive(Debug)]
pub struct Index {
    index: ScoredDictionary,
}

impl Index {
    pub fn new(metadata: &Metadata) -> Index {
        // Compute a vector of keyword, etext_no, simple score triples.
        let mut postings: Vec<(String,Etext,f64)> = Vec::new();
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
        for cls in equivalence_classes(&postings, &equal) {
            if cls.len() > 0 {
                let key = cls[0].0.clone();
                let etext_no = cls[0].1;
                let score = cls.iter().fold(0.0f64, |a,p| a + p.2);
                index.entry(key).or_insert(Vec::new()).push((etext_no,score));
            }
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
    while p < postings.len() {
        if r == results.len() || results[r].0 > postings[p].0 {
            results.insert( r, postings[p].clone() );
            r += 1;
            p += 1;
        } else if results[r].0 < postings[p].0 {
            r += 1;
        } else /* results[r].0 == postings[p].0 */ {
            results[r].1 += postings[p].1;
            r += 1;
            p += 1;
        }
    }
}

fn compare(left: &(String,Etext,f64), right: &(String,Etext,f64)) -> Ordering {
    match left.0.cmp(&right.0) {
        Ordering::Less =>    Ordering::Less,
        Ordering::Equal =>   left.1.cmp(&right.1),
        Ordering::Greater => Ordering::Greater,
    }
}

fn equal(left: &(String,Etext,f64), right: &(String,Etext,f64)) -> bool {
    match compare(left,right) {
        Ordering::Equal => true,
        _               => false,
    }
}

fn equivalence_classes<'a, T>(vector: &'a Vec<T>, pred: &Fn(&T,&T)->bool) -> Vec<&'a [T]>
    where T:PartialOrd {
        let mut result = Vec::new();
        if vector.len() > 0 {
            let mut i = 0;
            let mut j = 1;
            while j < vector.len() {
                if !pred(&vector[i], &vector[j]) {
                    result.push( &vector[i..j] );
                    i = j;
                }
                j += 1;
            }
            result.push( &vector[i..j] );
        }
        result
    }

// pub struct EQIter<T,I,F> where I:Iterator<Item=T>, F:Fn(&T,&T)->bool {
//     iterator: Peekable<I>,
//     last_seen: Option<T>,
//     predicate: F,
// }
// 
// impl<T,I,F> EQIter<T,I,F> where I:Iterator<Item=T>, F:Fn(&T,&T)->bool {
//     pub fn new(iter: I, pred: F) -> EQIter<T,I,F> {
//         EQIter { iterator: iter.peekable(), last_seen: None, predicate: pred }
//     }
// }
// 
// impl<T,I,F> Iterator for EQIter<T,I,F> where T:Clone, I:Iterator<Item=T>, F:Fn(&T,&T)->bool {
//     type Item = T;
// 
//     fn next(&mut self) -> Option<T> {
//         if self.last_seen.is_none() {
//             let n = self.iterator.next();
//             self.last_seen = n.clone();
//             n
//         } else {
//             let l: &T = self.last_seen.as_ref().unwrap();
//             let p = &self.predicate;
//             if self.iterator.peek().is_none() {
//                 None
//             } else if !p(l, self.iterator.peek().unwrap()) {
//                 None
//             } else {
//                 self.iterator.next()
//             }
//         }
//     }
// }
