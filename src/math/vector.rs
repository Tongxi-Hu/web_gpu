use std::ops::{Index, IndexMut};

use super::common::FuzzyEq;

#[derive(Copy, Clone, Debug)]
pub struct Vector<const D: usize> {
    data: [f32; D],
}

impl<const D: usize> Index<usize> for Vector<D> {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const D: usize> IndexMut<usize> for Vector<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const D: usize> FuzzyEq for Vector<D> {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        for i in 0..D {
            if self[i].fuzzy_eq(&other[i]) == false {
                return false;
            }
        }
        true
    }
}

impl<const D: usize> PartialEq for Vector<D> {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}
