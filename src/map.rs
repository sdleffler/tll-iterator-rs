use std::iter;
use std::marker::PhantomData;

use tll::ternary::{Nat, Pred, NatPred, Triple, NatTriple, Zero, One, Two};

use iterator::{Iterator, NonEmpty};

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

impl<L: NatPred, B, I: NonEmpty<Zero<L>>, F> NonEmpty<Zero<L>> for Map<Zero<L>, I, F>
    where I::Next: Iterator<Two<Pred<L>>>,
          F: FnMut(I::Item) -> B
{
    type Next = Map<Two<Pred<L>>, I::Next, F>;

    #[inline]
    fn next(mut self) -> (B, Map<Two<Pred<L>>, I::Next, F>) {
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

impl<L: NatPred + NatTriple, B, I: NonEmpty<One<L>>, F> NonEmpty<One<L>> for Map<One<L>, I, F>
    where I::Next: Iterator<Triple<L>, Item = I::Item>,
          F: FnMut(I::Item) -> B
{
    type Next = Map<Triple<L>, I::Next, F>;

    #[inline]
    fn next(mut self) -> (B, Map<Triple<L>, I::Next, F>) {
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

impl<L: NatPred, B, I: NonEmpty<Two<L>>, F> NonEmpty<Two<L>> for Map<Two<L>, I, F>
    where I::Next: Iterator<One<L>, Item = I::Item>,
          F: FnMut(I::Item) -> B
{
    type Next = Map<One<L>, I::Next, F>;

    #[inline]
    fn next(mut self) -> (B, Map<One<L>, I::Next, F>) {
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
