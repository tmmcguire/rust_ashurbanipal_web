//! Topic recommendation utilities.

// use std::collections::BitVec;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::path::Path;

use recommendation::{Etext,Recommendation};
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
            BufReader::new( File::open(path).unwrap() ).lines()
            .map( |line| {
                let line         = line.unwrap();
                let mut elements = line.split('\t');
                // The first element of each line is the etext number.
                let etext_no     = elements.next().unwrap().parse::<usize>().unwrap();
                // The remaining elements are common-noun bit numbers for the etext.
                let etext_data   = elements.map( |s| s.parse::<usize>().unwrap() ).collect();
                (etext_no, etext_data)
            } ).unzip();

        Topic {
            data           : vectors,
            etext_to_index : etexts.iter().cloned().enumerate().map(|(x,y)| (y,x)).collect(),
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
    fn scored_results(&self, etext_no : Etext) -> Option<Vec<(Etext,f64)>> {

        let row = match self.etext_to_index.get(&etext_no) {
            None      => return None,
            Some(idx) => &self.data[*idx],
        };

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
            .enumerate()
            .map( |(i,d)| (self.index_to_etext[i], d as f64) )
            .collect();

        Some(result)
    }
}
