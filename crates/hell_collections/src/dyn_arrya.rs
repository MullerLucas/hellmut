use std::{array, fmt::Debug};


pub struct DynArray<T, const SIZE: usize> {
    data: [T; SIZE],
    len: usize,
}

impl<T, const SIZE: usize> DynArray<T, SIZE>
    where [T; SIZE]: Default,
{
    pub fn from_default() -> Self {
        let data: [T; SIZE] = Default::default();
        Self::new(data, 0)
    }
}

impl<T, const SIZE: usize> DynArray<T, SIZE> {
    pub fn new(data: [T; SIZE], len: usize) -> Self {
        if len > SIZE {
            panic!("invalid length: '{}'", len);
        }

        Self {
            data,
            len,
        }
    }
    pub fn from_fn(cb: impl FnMut(usize) -> T) -> Self {
        let data = array::from_fn(cb);
        Self::new(data, 0)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == SIZE
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[0..self.len()]
    }

    pub fn push(&mut self, val: T) {
        if self.is_full() {
            panic!("trying to push into a full DynArray");
        }

        self.data[self.len] = val;
        self.len += 1;
    }

    pub fn set(&mut self, idx: usize, val: T) {
        if idx >= self.len() {
            panic!("trying to set invalid index '{}'", idx);
        }

        self.data[idx] = val;
    }

}

impl<T, const SIZE: usize> Debug for DynArray<T, SIZE>
    where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynArray").field("data", &self.data).field("len", &self.len).finish()
    }
}

impl<T, const SIZE: usize> Clone for DynArray<T, SIZE>
    where T: Clone
{
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), len: self.len.clone() }
    }
}
