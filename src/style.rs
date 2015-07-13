//! Part-of-speech and style recommendation utilities.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::ops::Add;
use std::path::Path;

use recommendation::{Etext,Recommendation};

/// Part-of-speech / style data
pub struct Style {
    /// Part-of-speech data, in matrix form.
    pub data           : Vec<Vec<f64>>,
    /// Map to convert etext number to index into data rows.
    pub etext_to_index : HashMap<Etext,usize>,
    /// Map to convert data row index into an etext number.
    pub index_to_etext : Vec<Etext>,
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
        let (etexts, vectors) : (Vec<Etext>,Vec<Vec<f64>>) =
            BufReader::new( File::open(path).unwrap() ).lines()
            .map( |line| {
                let line                 = line.unwrap();
                let mut elements         = line.split('\t');
                // The first element of each line is the etext number.
                let etext_no: Etext      = elements.next().unwrap().parse().unwrap();
                // The remaining elements are part-of-speech data for the etext.
                let etext_data: Vec<f64> = elements.map( |s| s.parse().unwrap() ).collect();
                (etext_no, etext_data)
            } ).unzip();

        Style {
            data           : vectors,
            // Create the mappings from vector index to etext number, and vice versa.
            etext_to_index : etexts.iter().cloned().enumerate().map(|(x,y)| (y,x)).collect(),
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
    fn scored_results(&self, etext_no : Etext) -> Option<Vec<(Etext,f64)>> {

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
fn distance(v1 : &Vec<f64>, v2 : &Vec<f64>) -> f64 {
    assert_eq!(v1.len(), v2.len());
    let sq = v1.iter()
        // Match each element with that from the other vector.
        .zip( v2.iter() )
        // Compute (elt1 - elt2)^2.
        .map( |(x,y)| f64::powi(x-y,2) )
        // Accumulate the value.
        .fold(0f64, Add::add);
    f64::sqrt(sq)
}
