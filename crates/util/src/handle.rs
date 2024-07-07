use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub struct Handle<T> {
    pub index: usize,
    pub _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(index: usize) -> Handle<T> {
        Self {
            index,
            _marker: PhantomData,
        }
    }
}

impl<T> Copy for Handle<T> {}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Eq for Handle<T> {}
impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}
