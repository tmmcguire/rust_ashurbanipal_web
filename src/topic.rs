//! Topic recommendation utilities.

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

// use std::collections::BitVec;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::path::Path;

use recommendation::{Etext,Recommendation,Score};
use mbitset::MBitSet;

/// Common nouns-based topic data.
pub struct Topic {
    /// Vector of the sets of nouns in each text.
    data : Vec<MBitSet>,
    /// Map to convert etext number to index into data rows.
    pub etext_to_index : HashMap<Etext,usize>,
    /// Map to convert data row index into an etext number.
    pub index_to_etext : Vec<Etext>,
}

impl Topic {

    /// Read the data file and construct a Topic object.
    ///
    /// The data file should be of the format:
    ///
    /// ```
    /// etext_no data
    /// ```
    /// 
    /// Elements on each line should be separated by tabs.
    ///
    ///
    /// # Panics
    ///
    /// This function will die if
    ///
    /// * The data file cannot be read.
    ///
    /// * The data file is not composed of tab-separated numbers.
    ///
    /// * The first element of each line is not an integer etext number.
    ///
    /// * The remaining elements are not integer values.
    ///
    /// # Examples
    ///
    /// ```
    /// let topic : Topic = Topic::read("data/gutenberg.nouns");
    /// ```
    ///
    pub fn read<P : AsRef<Path>>(path : P) -> Topic {
        let (etexts, vectors) : (Vec<Etext>,Vec<MBitSet>) =
            BufReader::new( panic_unless!("topic data", result: File::open(path)) ).lines()
            .map( |line| {
                let line            = panic_unless!("topic data", result: line);
                let mut elements    = line.split('\t');
                // The first element of each line is the etext number.
                let etext_no: Etext = panic_unless!("etext number",
                                                    option: elements.nth(0)
                                                    .and_then(|s| s.parse().ok())
                                                    );
                // The remaining elements are common-noun bit numbers for the etext.
                let etext_data = elements
                    .map( |s| panic_unless!("topic data", result: s.parse::<usize>()) )
                    .collect();
                (etext_no, etext_data)
            } ).unzip();

        Topic {
            data           : vectors,
            etext_to_index : etexts.iter()
                // duplicate etext_nos
                .cloned()
                // associate each etext_no with a row number
                .enumerate()
                // flip the pair, to map each etext_no to a row number
                .map(|(x,y)| (y,x))
                // collect into hashmap
                .collect(),
            index_to_etext : etexts,
        }
    }
}

impl Recommendation for Topic {
    /// Return a vector of (etext number, score) pairs if possible,
    /// based on the topic data. The vector will be sorted by
    /// etext_number; sorting it by score will be necessary before
    /// returning the recommendations.
    ///
    /// # Failures
    ///
    /// Returns None if the supplied etext number is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// let topic = Topic::read("gutenberg.pos");
    /// let results = topic.scored_results(773);
    /// ```
    ///
    /// `results` will be Some containing a vector of scores compared
    /// with etext number 773, Oscar Wilde's *Lord Arthur Savile's
    /// Crime and Other Stories*.
    fn scored_results(&self, etext_no : Etext) -> Option<Vec<(Etext,Score)>> {

        let row = match self.etext_to_index.get(&etext_no) {
            None      => return None,
            Some(idx) => &self.data[*idx],
        };

        // Randomly large MBitSet to avoid allocations below.
        let mut intersection = MBitSet::with_capacity(31265);
        let mut union = MBitSet::with_capacity(31265);

        let result = self.data.iter()
            .map(|vec| {
                // Jaccard distance. Inlined to avoid reallocating
                // intersection and union. There doesn't seem to be a
                // better way to do this.
                let intersection_card = intersection.set(row).and(vec).cardinality();
                let union_card = union.set(row).or(vec).cardinality();
                1f64 - (intersection_card as f64 / union_card as f64)
            })
            // match each row with row number
            .enumerate()
            // translate row numbers to etext_nos.
            .map( |(i,d)| (self.index_to_etext[i], d as Score) )
            .collect();

        Some(result)
    }
}
