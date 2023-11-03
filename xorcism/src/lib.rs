use std::borrow::Borrow;
use std::iter::Cycle;
use std::slice::Iter;
/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    m_key_cycle: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> { 
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a> 
        where Key: AsRef<[u8]> + ?Sized {
        Self { m_key_cycle: key.as_ref().iter().cycle() }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        let _= data.iter_mut()
            .zip(&mut self.m_key_cycle)
            .for_each(|(v, k)| *v ^= *k);
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b + Captures<'a> 
    where 
        Data: IntoIterator, 
        Data::IntoIter: 'b,
        Data::Item: Borrow<u8> {
        // this empty iterator silences a compiler complaint that
        // () doesn't implement ExactSizeIterator
        data.into_iter()
            .zip(&mut self.m_key_cycle)
            .map(move |(v, k)| v.borrow() ^ k)
    }

}

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}
