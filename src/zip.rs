use std::iter;
use std::marker::PhantomData;

use tll::ternary::{Nat, Pred, NatPred, Triple, NatTriple, Zero, One, Two};

use iterator::{Iterator, NonEmpty};


pub struct Zip<L: Nat, I: Iterator<L>, J: Iterator<L>> {
    phantom: PhantomData<L>,
    first: I,
    second: J,
}

impl<L: Nat, I: Iterator<L>, J: Iterator<L>> iter::IntoIterator for Zip<L, I, J> {
    type IntoIter = iter::Zip<I::IntoIter, J::IntoIter>;
    type Item = (I::Item, J::Item);

    fn into_iter(self) -> Self::IntoIter {
        self.first.into_iter().zip(self.second.into_iter())
    }
}

impl<L: Nat, I: Iterator<L>, J: Iterator<L>> Zip<L, I, J> {
    pub fn new(first: I, second: J) -> Zip<L, I, J> {
        Zip {
            phantom: PhantomData,
            first: first,
            second: second,
        }
    }
}

impl<L: Nat, I: Iterator<L>, J: Iterator<L>> Iterator<L> for Zip<L, I, J> {}

macro_rules! zip_impl {
    ($s:expr) => {
        {
            let s = $s;

            let (i, first) = s.first.next();
            let (j, second) = s.second.next();

            let next = Zip {
                phantom: PhantomData,
                first: first,
                second: second,
            };

            ((i, j), next)
        }
    };
}

impl<L: NatPred, I: NonEmpty<Zero<L>>, J: NonEmpty<Zero<L>>> NonEmpty<Zero<L>> for Zip<Zero<L>, I, J>
    where I::Next: Iterator<Two<Pred<L>>, Item = I::Item>,
          J::Next: Iterator<Two<Pred<L>>, Item = J::Item>
{
    type Next = Zip<Two<Pred<L>>, I::Next, J::Next>;

    fn next(self) -> (Self::Item, Self::Next) {
        zip_impl!(self)
    }
}

impl<L: NatPred + NatTriple, I: NonEmpty<One<L>>, J: NonEmpty<One<L>>> NonEmpty<One<L>> for Zip<One<L>, I, J>
    where I::Next: Iterator<Triple<L>, Item = I::Item>,
          J::Next: Iterator<Triple<L>, Item = J::Item>
{
    type Next = Zip<Triple<L>, I::Next, J::Next>;

    fn next(self) -> (Self::Item, Self::Next) {
        zip_impl!(self)
    }
}

impl<L: NatPred, I: NonEmpty<Two<L>>, J: NonEmpty<Two<L>>> NonEmpty<Two<L>> for Zip<Two<L>, I, J>
    where I::Next: Iterator<One<L>, Item = I::Item>,
          J::Next: Iterator<One<L>, Item = J::Item>
{
    type Next = Zip<One<L>, I::Next, J::Next>;

    fn next(self) -> (Self::Item, Self::Next) {
        zip_impl!(self)
    }
}
