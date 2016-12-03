use iterator::{Iterator, NonEmpty};

use std::iter;
use std::marker::PhantomData;

use tll::ternary::{Nat, Pred, NatPred};


pub struct Map<L: Nat, I: Iterator<L>, F> {
    phantom: PhantomData<L>,
    transform: F,
    iter: I,
}

impl<L: Nat, I: Iterator<L>, B, F> iter::IntoIterator for Map<L, I, F>
    where F: FnMut(I::Item) -> B
{
    type IntoIter = iter::Map<I::IntoIter, F>;
    type Item = B;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter.into_iter().map(self.transform)
    }
}

impl<L: Nat, I: Iterator<L>, F> Map<L, I, F> {
    pub fn new(iter: I, transform: F) -> Self {
        Map {
            phantom: PhantomData,
            iter: iter,
            transform: transform,
        }
    }
}

impl<L: Nat, I: Iterator<L>, B, F> Iterator<L> for Map<L, I, F> where F: FnMut(I::Item) -> B {}

impl<L: Nat, B, I: NonEmpty<L>, F> NonEmpty<L> for Map<L, I, F>
    where L: NatPred,
          F: FnMut(I::Item) -> B
{
    type Next = Map<Pred<L>, I::Next, F>;

    #[inline]
    fn next(mut self) -> (B, Map<Pred<L>, I::Next, F>) {
        let (a, next) = self.iter.next();
        let b = (self.transform)(a);

        let next = Map {
            phantom: PhantomData,
            transform: self.transform,
            iter: next,
        };

        (b, next)
    }
}
