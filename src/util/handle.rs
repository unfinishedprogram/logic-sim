use std::marker::PhantomData;

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
