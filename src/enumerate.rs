use std::iter;
use std::marker::PhantomData;
use std::ops::RangeFrom;

use tll::ternary::{Nat, Pred, NatPred, Succ, NatSucc, Triple, NatTriple, Term, Zero, One, Two};

use iterator::{Iterator, NonEmpty};


pub struct Enumerate<L: Nat, I: Iterator<L>, P: Nat> {
    phantom: PhantomData<(L, P)>,
    iter: I,
}

impl<L: Nat, I: Iterator<L>, P: Nat> iter::IntoIterator for Enumerate<L, I, P> {
    type IntoIter = iter::Zip<RangeFrom<usize>, I::IntoIter>;
    type Item = (usize, I::Item);

    fn into_iter(self) -> Self::IntoIter {
        (P::reify()..).zip(self.iter.into_iter())
    }
}

impl<L: Nat, I: Iterator<L>> Enumerate<L, I, Term> {
    pub fn new(iter: I) -> Enumerate<L, I, Term> {
        Enumerate {
            phantom: PhantomData,
            iter: iter,
        }
    }
}

impl<L: Nat, I: Iterator<L>, P: Nat> Iterator<L> for Enumerate<L, I, P> {}

macro_rules! enumerate_impl {
    ($s:expr) => {
        {
            let (a, iter) = $s.iter.next();

            let next = Enumerate {
                phantom: PhantomData,
                iter: iter,
            };

            ((P::reify(), a), next)
        }
    };
}

impl<L: NatPred, I: NonEmpty<Zero<L>>, P: NatSucc> NonEmpty<Zero<L>> for Enumerate<Zero<L>, I, P>
    where I::Next: Iterator<Two<Pred<L>>, Item = I::Item>
{
    type Next = Enumerate<Two<Pred<L>>, I::Next, Succ<P>>;

    fn next(self) -> ((usize, I::Item), Self::Next) {
        enumerate_impl!(self)
    }
}

impl<L: NatPred + NatTriple, I: NonEmpty<One<L>>, P: NatSucc> NonEmpty<One<L>> for Enumerate<One<L>, I, P>
    where I::Next: Iterator<Triple<L>, Item = I::Item>
{
    type Next = Enumerate<Triple<L>, I::Next, Succ<P>>;

    fn next(self) -> ((usize, I::Item), Self::Next) {
        enumerate_impl!(self)
    }
}

impl<L: NatPred, I: NonEmpty<Two<L>>, P: NatSucc> NonEmpty<Two<L>> for Enumerate<Two<L>, I, P>
    where I::Next: Iterator<One<L>, Item = I::Item>
{
    type Next = Enumerate<One<L>, I::Next, Succ<P>>;

    fn next(self) -> ((usize, I::Item), Self::Next) {
        enumerate_impl!(self)
    }
}
