//! Project Gutenberg etext recommendation utilities.

/// An etext number.
pub type Etext = usize;

pub trait Recommendation {
    /// Return a vector of (etext number, score) pairs if possible.
    /// The vector will be sorted by etext_number.
    fn scored_results(&self, etext_no : Etext) -> Option<Vec<(Etext,f64)>>;
}
