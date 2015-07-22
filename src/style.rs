//! Part-of-speech and style recommendation utilities.

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

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::ops::Add;
use std::path::Path;

use recommendation::{Etext,Recommendation,Score};

type Proportion = f64;

/// Part-of-speech / style data
pub struct Style {
    /// Part-of-speech data, in matrix form.
    data           : Vec<Vec<Proportion>>,
    /// Map to convert etext number to index into data rows.
    etext_to_index : HashMap<Etext,usize>,
    /// Map to convert data row index into an etext number.
    index_to_etext : Vec<Etext>,
}

impl Style {

    /// Read the data file and construct a Style object.
    ///
    /// The data file should be of the format:
    ///
    /// ```
    /// etext_no data...
    /// etext_no data...
    /// ...
    /// ```
    ///
    /// Elements on each line should be separated by tabs. The number
    /// of data elements on each line should be equal.
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
    /// * The remaining elements are not floating-point values.
    ///
    /// # Examples
    ///
    /// ```
    /// let style : Style = Style::read("gutenberg.pos");
    /// ```
    ///
    pub fn read<P : AsRef<Path>>(path : P) -> Style {
        let (etexts, vectors) : (Vec<Etext>,Vec<Vec<Score>>) =
            BufReader::new( panic_unless!("style data", result: File::open(path)) ).lines()
            .map( |line| {
                let line                 = panic_unless!("style_data", result: line);
                let mut elements         = line.split('\t');
                // The first element of each line is the etext number.
                let etext_no: Etext = panic_unless!("etext number",
                                                    option: elements.nth(0)
                                                    .and_then(|s| s.parse().ok())
                                                    );
                // The remaining elements are part-of-speech data for the etext.
                let etext_data: Vec<Proportion> = elements
                    .map( |s| panic_unless!("style data", result: s.parse::<Proportion>()) )
                    .collect();
                (etext_no, etext_data)
            } ).unzip();

        Style {
            data           : vectors,
            // Create the mappings from vector index to etext number, and vice versa.
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

impl Recommendation for Style {
    /// Return a vector of (etext number, score) pairs if possible,
    /// based on the part-of-speech data. The vector will be sorted by
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
    /// let style = Style::read("gutenberg.pos");
    /// let results = style.scored_results(773);
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

        let x = self.data.iter()
            // Compute the distance from row to v.
            .map( |v| distance(v,row) )
            // Associated each distance with its index.
            .enumerate()
            // Replace the index with the etext number.
            .map( |(i,d)| (self.index_to_etext[i], d) )
            // Create the result vector.
            .collect();

        Some(x)
    }
}

/// Compute the Euclidian distance between the two vectors.
fn distance(v1 : &Vec<Score>, v2 : &Vec<Score>) -> Score {
    assert_eq!(v1.len(), v2.len());
    let sq = v1.iter()
        // Match each element with that from the other vector.
        .zip( v2.iter() )
        // Compute (elt1 - elt2)^2.
        .map( |(x,y)| Score::powi(x-y,2) )
        // Accumulate the value.
        .fold(0 as Score, Add::add);
    Score::sqrt(sq)
}
