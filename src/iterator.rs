use std::iter;

use tll::ternary::{Nat, Pred, NatPred};


use chain::Chain;
use map::Map;


pub trait Iterator<L: Nat>: iter::IntoIterator {
    fn chain<M: Nat, U: Iterator<M, Item = Self::Item>>(self, other: U) -> Chain<L, M, Self, U>
        where Self: Sized
    {
        Chain::new(self, other)
    }

    fn map<B, F>(self, f: F) -> Map<L, Self, F>
        where F: FnMut(Self::Item) -> B,
              Self: Sized
    {
        Map::new(self, f)
    }

    fn collect_sized<B>(self) -> B
        where B: FromIterator<L, Self::Item>,
              Self: Sized
    {
        FromIterator::from_sized_iter(self)
    }
}

pub trait NonEmpty<L: Nat>: Iterator<L>
    where L: NatPred
{
    type Next: Iterator<Pred<L>, Item = Self::Item>;

    fn next(self) -> (Self::Item, Self::Next);
}

pub trait IntoIterator<L: Nat> {
    type IntoIter: Iterator<L, Item = Self::Item>;
    type Item;

    fn into_sized_iter(self) -> Self::IntoIter;
}

pub trait FromIterator<L: Nat, A> {
    fn from_sized_iter<I: Iterator<L, Item = A>>(I) -> Self;
}
