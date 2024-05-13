use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use super::NBT;

/// Compound represents a NBT compound which contains a mapping of Key-Value based values.
/// Key is always of type String and Value is of type NBT.
pub struct Compound {
    map: HashMap<String, NBT>,
}

impl Compound {
    /// Creates and returns a new Compound
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Gets a reference to the value at provided key.
    pub fn get(&self, key: &str) -> Option<&NBT> {
        let val = self.map.get(key)?;
        Some(val)
    }

    /// Gets a mutable reference to the value at provided key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut NBT> {
        let val = self.map.get_mut(key)?;
        Some(val)
    }

    /// Puts the provided NBT object into the map at provided key.
    pub fn put(&mut self, key: &str, nbt: NBT) {
        self.map.insert(key.to_string(), nbt);
    }
}

/*
    Creates and returns a Compound NBT. Provided below is an example use case.

    # Example

    ```
    let compound = compound![
        "byte" => 120_i8,
        "map" => compound!(
            "integer" => 100_i32,
            "string" => "hello world"
        )
    ];
    ```
*/
#[macro_export]
macro_rules! compound {
    ($($key:expr => $value:expr),*) => {{
        let mut map = Compound::new();
        $(
            map.put($key, $value.into());
        )*
        NBT::Compound(map)
    }};
}

impl Debug for Compound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.map)
    }
}

impl Deref for Compound {
    type Target = HashMap<String, NBT>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Compound {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
