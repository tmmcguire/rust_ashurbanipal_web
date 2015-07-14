//! Simple bit vector until std::collections::BitVec stabilizes.

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

#![allow(dead_code)]

use std::iter::FromIterator;

/// Type of elements used to store bits.
type Elt = u64;
/// Size of elements used to store bits.
const ESIZE : usize = 64;

/// A set of bits, modeled as a vector of elements.
pub struct MBitSet {
    storage : Vec<Elt>,
}

impl MBitSet {

    /// Create an empty MBitSet.
    pub fn new() -> MBitSet {
        MBitSet { storage : Vec::new(), }
    }

    /// Create a MBitSet with a specified capacity.
    pub fn with_capacity(cap : usize) -> MBitSet {
        let elements = cap / ESIZE;
        let mut bs = MBitSet { storage : Vec::with_capacity(elements) };
        for _ in 0..elements { bs.storage.push(0); }
        bs
    }

    pub fn to_string(&self) -> String {
        format!("MBit ({:?})", self.storage)
    }

    pub fn clear(&mut self) {
        for i in self.storage.iter_mut() { *i = 0; }
    }

    pub fn set(&mut self, other : &MBitSet) -> &mut Self {
        let slen = self.storage.len();
        let olen = other.storage.len();
        if slen < olen {
            self.extend(olen);
        }
        for i in 0..olen {
            self.storage[i] = other.storage[i];
        }
        for i in olen..slen {
            self.storage[i] = 0;
        }
        self
    }

    pub fn contains(&self, b : usize) -> bool {
        let (elt,bit) = elt_pair(b);
        if self.storage.len() <= elt {
            false
        } else {
            (self.storage[elt] & bit) == bit
        }
    }

    pub fn insert(&mut self, b : usize) -> bool {
        if self.contains(b) {
            false
        } else {
            let (elt,bit) = elt_pair(b);
            let len = self.storage.len();
            if len <= elt {
                self.extend(elt + 1);
            }
            self.storage[elt] |= bit;
            true
        }
    }

    pub fn and(&mut self, other : &MBitSet) -> &mut Self {
        let mut len = self.storage.len();
        if other.storage.len() < len {
            len = other.storage.len();
        }
        for i in 0..len {
            self.storage[i] &= other.storage[i];
        }
        self
    }

    pub fn or(&mut self, other : &MBitSet) -> &mut Self {
        if other.storage.len() > self.storage.len() {
            self.extend( other.storage.len() );
        }
        for i in 0..other.storage.len() {
            self.storage[i] |= other.storage[i];
        }
        self
    }

    pub fn cardinality(&self) -> usize {
        self.storage.iter()
            .fold(0, |acc,i| {
                let mut x : Elt = *i;
                let mut count = 0;
                while x != 0 {
                    x &= x-1;
                    count += 1;
                }
                acc + count
            })
    }

    fn extend(&mut self, elt : usize) {
        let len = self.storage.len();
        self.storage.reserve(elt - len);
        for _ in len..elt {
            self.storage.push(0);
        }
    }
}

impl FromIterator<usize> for MBitSet {
    fn from_iter<I : IntoIterator<Item=usize>>(iter : I) -> MBitSet {
        let mut bitvec = MBitSet::new();
        for i in iter { bitvec.insert(i); }
        bitvec
    }
}

fn elt_pair(b : usize) -> (usize, Elt) { (b / ESIZE, 1 << (b % ESIZE)) }

#[test]
fn t1() {
    let bv = MBitSet::new();
    assert!(!bv.contains(0));
    assert!(!bv.contains(1));
    assert!(!bv.contains(128));
}

#[test]
fn t2() {
    let mut bv = MBitSet::new();
    bv.insert(1);
    assert!(!bv.contains(0));
    assert!(bv.contains(1));
    assert!(!bv.contains(128));
}

#[test]
fn t3() {
    let mut bv = MBitSet::new();
    bv.insert(64);
    assert!(!bv.contains(0));
    assert!(bv.contains(64));
    println!("{}", bv.to_string());
}

#[test]
fn t4() {
    let bv : MBitSet = [1usize,2,3,128].iter().map(|&x| x).collect();
    assert!(bv.contains(1));
    assert!(bv.contains(2));
    assert!(bv.contains(3));
    assert!(bv.contains(128));
    assert!(!bv.contains(0));
    assert!(!bv.contains(4));
    assert!(!bv.contains(127));
    assert!(!bv.contains(129));
}

#[test]
fn t5() {
    let v1 : MBitSet = [1usize,2,3,128].iter().map(|&x| x).collect();
    let mut v2 = MBitSet::new();
    v2.and(&v1);
    assert!(!v2.contains(1));
    assert!(!v2.contains(2));
    assert!(!v2.contains(3));
    assert!(!v2.contains(128));
}

#[test]
fn t6() {
    let v1 : MBitSet = [1usize,2,3,128].iter().map(|&x| x).collect();
    let mut v2 = MBitSet::new();
    v2.or(&v1);
    assert!(v2.contains(1));
    assert!(v2.contains(2));
    assert!(v2.contains(3));
    assert!(v2.contains(128));
}

#[test]
fn t7() {
    let v1 : MBitSet = [1usize,2,3,128].iter().map(|&x| x).collect();
    let mut v2 : MBitSet = [1usize,2,3,128].iter().map(|&x| x).collect();
    v2.and(&v1);
    assert!(v2.contains(1));
    assert!(v2.contains(2));
    assert!(v2.contains(3));
    assert!(v2.contains(128));
}
