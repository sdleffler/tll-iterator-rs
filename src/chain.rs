use std::iter;
use std::marker::PhantomData;

use tll::ternary::{Nat, Pred, NatPred, Triple, NatTriple, Add, NatAdd, Term, Zero, One, Two};

use iterator::Iterator;
use iterator::NonEmpty;

pub struct Chain<A: Nat, B: Nat, I: Iterator<A>, J: Iterator<B, Item = I::Item>> {
    phantom: PhantomData<(A, B)>,
    first: I,
    second: J,
}

impl<A: Nat, B: Nat, I: Iterator<A>, J: Iterator<B, Item = I::Item>> iter::IntoIterator for Chain<A, B, I, J> {
    type IntoIter = iter::Chain<I::IntoIter, J::IntoIter>;
    type Item = I::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.first.into_iter().chain(self.second.into_iter())
    }
}

impl<A: Nat, B: Nat, I: Iterator<A>, J: Iterator<B, Item = I::Item>> Chain<A, B, I, J> {
    pub fn new(first: I, second: J) -> Self {
        Chain {
            phantom: PhantomData,
            first: first,
            second: second,
        }
    }
}

impl<A: NatAdd<B>, B: Nat, I: Iterator<A>, J: Iterator<B, Item = I::Item>> Iterator<Add<A, B>> for Chain<A, B, I, J> {}

macro_rules! chain_impl_ret_first {
    () => {
        #[inline]
        fn next(self) -> (I::Item, Self::Next) {
            let Chain { first, second, .. } = self;
            let (a, first) = first.next();
            let next = Chain {
                phantom: PhantomData,
                first: first,
                second: second,
            };
            (a, next)
        }
    };
}

macro_rules! chain_impl_ret_second {
    () => {
        #[inline]
        fn next(self) -> (I::Item, Self::Next) {
            let Chain { first, second, .. } = self;
            let (b, second) = second.next();
            let next = Chain {
                phantom: PhantomData,
                first: first,
                second: second,
            };
            (b, next)
        }
    };
}

impl<A: NatPred, B: Nat, I: NonEmpty<Zero<A>>, J: Iterator<B, Item = I::Item>> NonEmpty<Add<Zero<A>, B>> for Chain<Zero<A>, B, I, J>
    where Zero<A>: NatAdd<B>,
          Pred<Zero<A>>: NatAdd<B, Output = Pred<Add<Zero<A>, B>>>,
          Add<Zero<A>, B>: NatPred,
          I::Next: Iterator<Pred<Zero<A>>, Item = I::Item>
{
    type Next = Chain<Pred<Zero<A>>, B, I::Next, J>;

    chain_impl_ret_first!();
}

impl<A: NatTriple + NatPred, B: Nat, I: NonEmpty<One<A>>, J: Iterator<B, Item = I::Item>> NonEmpty<Add<One<A>, B>> for Chain<One<A>, B, I, J>
    where One<A>: NatAdd<B>,
          Pred<One<A>>: NatAdd<B, Output = Pred<Add<One<A>, B>>>,
          Add<One<A>, B>: NatPred,
          I::Next: Iterator<Pred<One<A>>, Item = I::Item>
{
    type Next = Chain<Pred<One<A>>, B, I::Next, J>;

    chain_impl_ret_first!();
}

impl<A: NatPred, B: Nat, I: NonEmpty<Two<A>>, J: Iterator<B, Item = I::Item>> NonEmpty<Add<Two<A>, B>> for Chain<Two<A>, B, I, J>
    where Two<A>: NatAdd<B>,
          Pred<Two<A>>: NatAdd<B, Output = Pred<Add<Two<A>, B>>>,
          Add<Two<A>, B>: NatPred,
          I::Next: Iterator<Pred<Two<A>>, Item = I::Item>
{
    type Next = Chain<Pred<Two<A>>, B, I::Next, J>;

    chain_impl_ret_first!();
}

impl<B: NatTriple + NatPred, I: Iterator<Term>, J: NonEmpty<Zero<B>, Item = I::Item>> NonEmpty<Add<Term, Zero<B>>> for Chain<Term, Zero<B>, I, J>
    where Zero<B>: NatPred,
          Triple<B>: NatPred,
          Term: NatAdd<B> + NatAdd<Pred<Zero<B>>, Output = Pred<Add<Term, Zero<B>>>>,
          Add<Term, Zero<B>>: NatPred,
          J::Next: Iterator<Pred<Zero<B>>, Item = J::Item>
{
    type Next = Chain<Term, Pred<Zero<B>>, I, J::Next>;

    chain_impl_ret_second!();
}

impl<B: NatTriple + NatPred, I: Iterator<Term>, J: NonEmpty<One<B>, Item = I::Item>> NonEmpty<Add<Term, One<B>>> for Chain<Term, One<B>, I, J>
    where One<B>: NatPred,
          Triple<B>: NatPred,
          Term: NatAdd<B> + NatAdd<Pred<One<B>>, Output = Pred<Add<Term, One<B>>>>,
          Add<Term, One<B>>: NatPred,
          J::Next: Iterator<Pred<One<B>>, Item = J::Item>
{
    type Next = Chain<Term, Pred<One<B>>, I, J::Next>;

    chain_impl_ret_second!();
}

impl<B: NatTriple + NatPred, I: Iterator<Term>, J: NonEmpty<Two<B>, Item = I::Item>> NonEmpty<Add<Term, Two<B>>> for Chain<Term, Two<B>, I, J>
    where Two<B>: NatPred,
          Triple<B>: NatPred,
          Term: NatAdd<B> + NatAdd<Pred<Two<B>>, Output = Pred<Add<Term, Two<B>>>>,
          Add<Term, Two<B>>: NatPred,
          J::Next: Iterator<Pred<Two<B>>, Item = J::Item>
{
    type Next = Chain<Term, Pred<Two<B>>, I, J::Next>;

    chain_impl_ret_second!();
}
