use crate::HashMap;
use eyre::eyre;
use num_traits::PrimInt;
/// Extensions to some standard library types, such as Iterator and Option.
use std::hash::Hash;

pub trait Frequencies<FreqType: PrimInt>: Iterator {
    fn frequencies(self) -> HashMap<Self::Item, FreqType>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = HashMap::default();
        self.for_each(|item| {
            let entry = counts.entry(item).or_insert_with(FreqType::zero);
            *entry = entry.add(FreqType::one());
        });
        counts
    }
}

impl<It: ?Sized, FreqType: PrimInt> Frequencies<FreqType> for It where It: Iterator {}

pub trait MoreItertools: Iterator {
    #[inline]
    fn take_until<P>(self, predicate: P) -> TakeUntil<Self, P>
    where
        Self: Sized,
    {
        TakeUntil {
            it: self,
            flag: false,
            predicate,
        }
    }
}

impl<I: Iterator> MoreItertools for I {}

pub struct TakeUntil<I, P> {
    it: I,
    predicate: P,
    flag: bool,
}

impl<I: Iterator, P> Iterator for TakeUntil<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let x = self.it.next()?;

        if self.flag {
            return None;
        }

        if (self.predicate)(&x) {
            self.flag = true;
        }

        Some(x)
    }
}

pub trait OptionTools<T> {
    fn ok_or_parse_error(self) -> crate::Result<T>;
}

impl<T> OptionTools<T> for Option<T> {
    fn ok_or_parse_error(self) -> crate::Result<T> {
        self.ok_or(eyre!("parse error"))
    }
}
