//! Combined style and topic recommendations.

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

use recommendation::{Etext,Recommendation,Score};

/// Combined recommendations data
pub struct Combination<'a> {
    left : &'a Recommendation,
    right : &'a Recommendation,
}

impl<'a> Combination<'a> {

    /// Construct a new Combination structure based on existing
    /// `Style` and `Type` objects.
    pub fn new(left : &'a Recommendation, right : &'a Recommendation) -> Combination<'a> {
        Combination { left : left, right : right }
    }

}

impl<'a> Recommendation for Combination<'a> {

    /// Return a vector of (etext number, score) pairs if possible,
    /// based on the combined recommendation data. The vector will be
    /// sorted by etext_number; sorting it by score will be necessary
    /// before returning the recommendations.
    ///
    /// # Failures
    ///
    /// Returns None if the supplied etext number is not valid.
    ///
    /// If an etext is not in both recommendation lists, it will be
    /// skipped in the combined list.
    ///
    /// # Examples
    ///
    /// ```
    /// let combination = Combination::new(style,topic);
    /// let results = combination.scored_results(773);
    /// ```
    ///
    /// `results` will be Some containing a vector of scores compared
    /// with etext number 773, Oscar Wilde's *Lord Arthur Savile's
    /// Crime and Other Stories*.
    fn scored_results(&self, etext_no : Etext) -> Option<Vec<(Etext,Score)>> {
        match (self.left.scored_results(etext_no), self.right.scored_results(etext_no)) {
            (Some(left), Some(right)) => {

                let mut results = Vec::with_capacity(left.len());

                let mut lefts = left.iter();
                let mut next_left = lefts.next();
                let mut rights = right.iter();
                let mut next_right = rights.next();

                loop {
                    match (next_left, next_right) {
                        (Some(&(sn,ss)), Some(&(tn,ts))) => {
                            if      sn < tn { next_left = lefts.next(); }
                            else if tn < sn { next_right = rights.next(); }
                            else {
                                results.push((sn, ss * ts));
                                next_left = lefts.next();
                                next_right = rights.next();
                            }
                        }
                        _ => break
                    }
                }
                Some(results)
            }
            (_,_) => None
        }
    }

}

