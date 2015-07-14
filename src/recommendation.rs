//! Project Gutenberg etext recommendation utilities.

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

/// An etext number.
pub type Etext = usize;
/// Ranking score.
pub type Score = f64;

pub trait Recommendation : Sync {
    /// Return a vector of (etext number, score) pairs if possible.
    /// The vector will be sorted by etext_number.
    fn scored_results(&self, etext_no : Etext) -> Option<Vec<(Etext,Score)>>;

    /// Return a vector of (etext number, score) pairs if possible,
    /// sorted by score.
    fn sorted_results(&self, etext_no : Etext) -> Option<Vec<(Etext,Score)>> {
        self.scored_results(etext_no).map( |mut results| {
            results.sort_by( |&(_,l),&(_,r)| l.partial_cmp(&r).unwrap() );
            results
        })
    }
}
