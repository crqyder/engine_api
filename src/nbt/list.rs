use std::fmt::Debug;

use super::{Tag, NBT};

/// List represents a collection of NBT objects. It is a homogenous collection of NBT objects, in other
/// words, objects of same type only.
pub struct List {
    id: Tag,
    list: Vec<NBT>,
}

impl List {
    /// Creates and returns a new List object with the provided type of objects
    pub fn new(id: Tag) -> Self {
        Self {
            id,
            list: Vec::new(),
        }
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
        if self.id != nbt.id() {
            return;
        }

        self.list.push(nbt)
    }

    /// Inserts the provided NBT object at the provided index.
    pub fn insert(&mut self, index: usize, nbt: NBT) {
        if self.id != nbt.id() {
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

/*
    Implement Debug trait for List object for pretty printing the inner values.
*/
impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.list)
    }
}
