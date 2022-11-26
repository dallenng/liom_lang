use std::convert::TryInto;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Arena<T> {
    data: Vec<T>,
}

impl<T> Arena<T> {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn alloc(&mut self, value: T) -> Idx<T> {
        let i = self.next_index();
        self.data.push(value);

        i
    }

    fn next_index(&self) -> Idx<T> {
        self.data.len().into()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Idx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, index: Idx<T>) -> &Self::Output {
        &self.data[usize::from(index)]
    }
}

impl<T> IndexMut<Idx<T>> for Arena<T> {
    fn index_mut(&mut self, index: Idx<T>) -> &mut Self::Output {
        &mut self.data[usize::from(index)]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Idx<T> {
    raw: u32,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> From<usize> for Idx<T> {
    fn from(raw: usize) -> Self {
        Self { raw: raw.try_into().expect("too many elements"), _phantom: PhantomData }
    }
}

impl<T> From<Idx<T>> for usize {
    fn from(i: Idx<T>) -> Self {
        i.raw as usize
    }
}

impl<T> fmt::Debug for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut type_name = std::any::type_name::<T>();

        if let Some(i) = type_name.rfind(':') {
            type_name = &type_name[i + 1..];
        }
        write!(f, "Idx::<{type_name}>({})", self.raw)
    }
}

impl<T> fmt::Display for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.raw.fmt(f)
    }
}
