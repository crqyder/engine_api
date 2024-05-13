use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use super::{Tag, NBT};

/// List represents a collection of NBT objects. It is a homogenous collection of NBT objects, in other
/// words, objects of same type only.
#[derive(PartialEq, Clone)]
pub struct List {
    tag: Tag,
    list: Vec<NBT>,
}

impl List {
    /// Creates and returns a new List object with the provided type of objects
    pub fn new(tag: Tag) -> Self {
        Self {
            tag,
            list: Vec::new(),
        }
    }

    /// Creates and returns a new List object with the provided type and the specified
    /// capacity.
    pub fn with_capacity(tag: Tag, cap: usize) -> Self {
        Self {
            tag,
            list: Vec::with_capacity(cap),
        }
    }

    /// Returns the type of NBT objects contained by the List.
    pub fn tag(&self) -> Tag {
        self.tag
    }

    /// Gets a reference to the value at provided index.
    pub fn get(&self, index: usize) -> Option<&NBT> {
        let val = self.list.get(index)?;
        Some(val)
    }

    /// Gets a mutable reference to the value at provided index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut NBT> {
        let val = self.list.get_mut(index)?;
        Some(val)
    }

    /// Puts the provided NBT object at the end of the list.
    pub fn put(&mut self, nbt: NBT) {
        if self.tag != nbt.tag() {
            return;
        }

        self.list.push(nbt)
    }

    /// Inserts the provided NBT object at the provided index.
    pub fn insert(&mut self, index: usize, nbt: NBT) {
        if self.tag != nbt.tag() {
            return;
        }

        self.list.insert(index, nbt)
    }
}

/*
    Creates and returns a List NBT. Provided below is an example use case.

    # Example

    ```
    list![Tag::Short, 1, 2, 3];
    ```
*/
#[macro_export]
macro_rules! list {
    ($NBT_id:expr, $($NBTs:expr),*) => {{
        let mut list = List::new($NBT_id);
        $(list.list.push($NBTs.into());)*
        NBT::List(list)
    }};
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.list)
    }
}

impl Deref for List {
    type Target = Vec<NBT>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}
