use iterator::{Iterator, NonEmpty};

use std::iter;
use std::marker::PhantomData;

use tll::number::ternary::{Nat, Pred, NatPred};


pub struct Map<L: Nat, I: Iterator<L>, B, F: FnMut(I::Item) -> B> {
    phantom: PhantomData<L>,
    transform: F,
    iter: I,
}

impl<L: Nat, I: Iterator<L>, B, F: FnMut(I::Item) -> B> Iterator<L> for Map<L, I, B, F> {
    type Item = B;
}

impl<L: Nat, I: NonEmpty<L>, B, F: FnMut(I::Item) -> B> NonEmpty<L> for Map<L, I, B, F>
    where L: NatPred
{
    type Next = Map<Pred<L>, I::Next, B, F>;

    #[inline]
    fn next(mut self) -> (B, Map<Pred<L>, I::Next, B, F>) {
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


impl<L: Nat, I, B, F> iter::IntoIterator for Map<L, I, B, F>
    where I: IntoIterator + Iterator<L, Item = <I as IntoIterator>::Item>,
          F: FnMut(<I as Iterator<L>>::Item) -> B
{
    type IntoIter = iter::Map<I::IntoIter, F>;
    type Item = B;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter.into_iter().map(self.transform)
    }
}
