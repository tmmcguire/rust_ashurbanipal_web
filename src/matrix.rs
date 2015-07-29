use std::iter::Iterator;
use std::ops::Index;

#[derive(Debug)]
pub struct Matrix2<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Matrix2<T> {
    pub fn new(data: &[Vec<T>]) -> Matrix2<T> {
        let rows = data.len();
        let cols = data[0].len();
        assert!(rows > 0);
        assert!(cols > 0);
        let mut v = Vec::with_capacity(rows * cols);
        for row in data.iter() {
            for col in row.iter() {
                v.push( col.clone() );
            }
        }
        Matrix2 { data: v, rows: rows, cols: cols }
    }

    pub fn rows<'m>(&'m self) -> RowsIter<'m,T> {
        RowsIter { matrix: self, cur: 0 }
    }
}

impl<T> Index<usize> for Matrix2<T> {
    type Output = [T];
    
    fn index<'m>(&'m self, index: usize) -> &'m Self::Output {
        let start = self.cols * index;
        assert!(start + self.cols <= self.data.len());
        &self.data[(start..start + self.cols)]
    }
}

pub struct RowsIter<'m,T: 'm> {
    matrix: &'m Matrix2<T>,
    cur: usize,
}

impl<'m,T> Iterator for RowsIter<'m,T> {
    type Item = &'m [T];

    fn next(&mut self) -> Option<&'m [T]> {
        if self.cur >= self.matrix.data.len() { None }
        else {
            let cols = self.matrix.cols;
            let offset = self.cur;
            self.cur += cols;
            Some(&self.matrix.data[(offset..offset + cols)])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.matrix.rows - (self.cur / self.matrix.cols);
        (size, Some(size))
    }
}


#[test]
fn test1() {
    let vec = vec!( vec!(1,2,3,4), vec!(5,6,7,8) );
    let x = Matrix2::new(&vec);
    for col in 0..4 {
        assert_eq!(x[0][col], [1,2,3,4][col]);
        assert_eq!(x[1][col], [5,6,7,8][col]);
    }
    assert_eq!(x.rows().count(), 2);
}
