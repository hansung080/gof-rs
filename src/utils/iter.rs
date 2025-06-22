use std::marker::PhantomData;

pub struct NoneIterator<T> {
    _marker: PhantomData<T>,
}

impl<T> NoneIterator<T> {
    pub fn new() -> Self {
        NoneIterator { _marker: PhantomData }
    }
}

impl<T> Iterator for NoneIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}