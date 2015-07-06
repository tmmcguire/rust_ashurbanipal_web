//! Project Gutenberg etext recommendation utilities.

/// An etext number.
pub type Etext = usize;

pub trait Recommendation {
    /// Return a vector of (etext number, score) pairs if possible.
    /// The vector will be sorted by etext_number.
    fn scored_results(&self, etext_no : Etext) -> Option<Vec<(Etext,f64)>>;

    /// Return a vector of (etext number, score) pairs if possible,
    /// sorted by score.
    fn sorted_results(&self, etext_no : Etext) -> Option<Vec<(Etext,f64)>> {
        match self.scored_results(etext_no) {
            None => None,
            Some(mut results) => {
                results.sort_by( |&(_,l),&(_,r)| l.partial_cmp(&r).unwrap() );
                Some(results)
            }
        }
    }
}
