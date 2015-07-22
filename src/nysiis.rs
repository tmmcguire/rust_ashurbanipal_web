//! New York State Identification and Intelligence System Phonetic Code
//!
//! See https://en.wikipedia.org/wiki/New_York_State_Identification_and_Intelligence_System

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

use iterator_utilities::buffer::IteratorBuffer;

pub fn encode(s: &str) -> String {
    Processor::new(s.chars()
                   .filter( |ch| ch.is_alphabetic() )
                   .flat_map( |ch| ch.to_lowercase() )
                   ).collect()
}

struct Processor<I> where I: Iterator, I::Item: Clone {
    iter: IteratorBuffer<I>,
    last: Option<I::Item>,
}

impl<I:Iterator<Item=char>> Processor<I> {
    pub fn new(iter: I) -> Processor<I> {
        Processor {
            iter: IteratorBuffer::new(iter, 3),
            last: None,
        }
    }


    fn openings(&mut self) {
        // Translate first characters of name: MAC → MCC, KN → N,
        // K → C, PH, PF → FF, SCH → SSS
        for &(prefix,replacement) in PREFIXES {
            if self.iter.starts_with(prefix) {
                self.iter.replace(prefix.len(), replacement);
                break;
            }
        }
    }

    fn closings(&mut self) {
        // Translate last characters of name: EE → Y, IE → Y, DT, RT,
        // RD, NT, ND → D
        for &(suffix,replacement) in SUFFIXES {
            if self.iter.ends_with(suffix) {
                self.iter.replace(suffix.len(), replacement);
                break;
            }
        }
    }
    
    fn transcoding(&mut self) {
        self.base_rules();
        self.h_rule();
        self.w_rule();
        self.terminal();
    }

    fn base_rules(&mut self) {
        // Translate remaining characters by following rules,
        // incrementing by one character each time:
        // * EV → AF else A, E, I, O, U → A
        // * Q → G, Z → S, M → N
        // * KN → N else K → C
        // * SCH → SSS, PH → FF
        'rule: for &rule in TRANSLATIONS {
            for &(pattern,replacement) in rule {
                if self.iter.len() >= pattern.len()
                    && self.iter.buffer().starts_with(pattern) {
                        self.iter.replace(pattern.len(), replacement);
                        break 'rule;
                    }
            }
        }
    }        

    fn h_rule(&mut self) {
        // * H → If previous or next is non-vowel, previous.
        if self.iter[0] == 'h' {
            // Note: !opening -> last == Some(_)
            let last = panic_unless!("nysiis opening", option: self.last);
            if !is_vowel(&last) || (self.iter.len() > 1 && !is_vowel(&self.iter[1])) {
                self.iter[0] = last;
            }
        }
    }

    fn w_rule(&mut self) {
        // * W → If previous is vowel, A.
        if self.iter[0] == 'w' {
            // Note: !opening -> last == Some(_)
            let last = panic_unless!("nysiis opening", option: self.last);
            if is_vowel(&last) {
                self.iter[0] = 'a';
            }
        }
    }

    fn terminal(&mut self) {
        if self.iter.is_closing() {
            if self.iter.len() == 2 {
                if self.iter.buffer() == &['a','s'] {
                    // If last characters are AS, remove them.
                    self.iter.replace(2, &[]);
                } else if self.iter.buffer() == &['a','y'] {
                    // If last characters are AY, replace with Y.
                    self.iter.replace(2, &['y']);
                }
            }
            if self.iter.len() == 1 && (self.iter[0] == 's' || self.iter[0] == 'a') {
                // If last character is S, remove it.
                self.iter.replace(1, &[]);
            }
        }
    }
}

impl<I:Iterator<Item=char>> Iterator for Processor<I> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.iter.is_opening() {
            self.openings();
        }
        if self.iter.is_closing() {
            self.closings();
        }
        // First character of key = first character of name.
        if !self.iter.is_opening() && self.iter.len() > 0 {
            self.transcoding();
        }
        match self.iter.pop() {
            Some(ch) if Some(ch) == self.last => { self.next() }
            Some(ch) => { self.last = Some(ch); self.last }
            None => { self.last = None; self.last }
        }
    }
}

const VOWELS: &'static [char] = &['a','e','i','o','u'];

fn is_vowel(s: &char) -> bool { VOWELS.contains(s) }

const PREFIXES: &'static [(&'static [char], &'static [char])] = &[
    (&['m','a','c'], &['m','c','c']),
    (&['k','n'],     &['n']),
    (&['k'],         &['c']),
    (&['p','h'],     &['f','f']),
    (&['p','f'],     &['f','f']),
    (&['s','c','h'], &['s','s','s']),
    ];

const SUFFIXES: &'static [(&'static [char], &'static [char])] = &[
    (&['e','e'], &['y']),
    (&['i','e'], &['y']),
    (&['d','t'], &['d']),
    (&['r','t'], &['d']),
    (&['r','d'], &['d']),
    (&['n','t'], &['d']),
    (&['n','d'], &['d'])
    ];

const TRANSLATIONS: &'static [&'static [(&'static [char], &'static [char])]] = &[
    &[
        (&['e','v'],     &['a','f']),
        (&['a'],         &['a']),
        (&['e'],         &['a']),
        (&['i'],         &['a']),
        (&['o'],         &['a']),
        (&['u'],         &['a']),
        ],
    &[
        (&['q'],         &['g']),
        (&['z'],         &['s']),
        (&['m'],         &['n']),
        ],
    &[
        (&['k','n'],     &['n']),
        (&['k'],         &['c']),
        ],
    &[
        (&['s','c','h'], &['s','s','s']),
        (&['p','h'],     &['f','f']),
        ],
    ];

#[test]
fn test1() {
    assert_eq!(encode("macbeth"),    "mcbat");
    assert_eq!(encode("knuth"),      "nat");
    assert_eq!(encode("kirk"),       "carc");
    assert_eq!(encode("phineas"),    "fana");
    assert_eq!(encode("pfaust"),     "fast");
    assert_eq!(encode("schwindler"), "swandlar");
}

#[test]
fn test2() {
    assert_eq!(encode("levee"),   "lafy");
    assert_eq!(encode("cookie"),  "cacy");
    assert_eq!(encode("fondt"),   "fand");
    assert_eq!(encode("yogurt"),  "yagad");
    assert_eq!(encode("word"),    "wad");
    assert_eq!(encode("valiant"), "valad");
    assert_eq!(encode("viand"),   "vad");
}

#[test]
fn test3() {
    assert_eq!(encode("pequant"), "pagad");
    assert_eq!(encode("lazy"),    "lasy");
    assert_eq!(encode("yammer"),  "yanar");
    assert_eq!(encode("aha"),     "ah");
}

#[test]
fn test4() {
    assert_eq!(encode("brown"),       "bran");
    assert_eq!(encode("browne"),      "bran");
    assert_eq!(encode("shakespeare"), "sacaspar");
    assert_eq!(encode("shakespear"),  "sacaspar");
}
