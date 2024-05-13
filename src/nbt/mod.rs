pub mod compound;
pub use compound::*;

pub mod list;
pub use list::*;

use std::fmt::Debug;

macro_rules! generate_nbt {
    ($($variant:ident => $type:ty, $as:ident, $as_mut:ident),*) => {
        /// Tag is an enumeration of NBT Tags. It is used as an identifier for reading and writing NBT objects
        /// to identify what type of [`NBT`] object we are reading or writing.
        #[derive(Debug, PartialEq)]
        #[repr(u8)]
        pub enum Tag {
            End,
            $($variant,)*
        }

        /// NBT is an implementation of a NBT Object. Each NBT Object has a unique identifier [`Tag`] associated
        /// with it. NBT is an enumeration of various wrappers for data types that can be written or read over NBT.
        /// From trait is implemented for these inner types to form a [`NBT`] object and vice-versa for easier conversions.
        pub enum NBT {
            $($variant($type),)*
        }

        impl NBT {
            /// Returns the Tag of the NBT object.
            pub fn tag(&self) -> Tag {
                match self {
                    $(NBT::$variant(_) => Tag::$variant,)*
                }
            }
        }

        $(
            impl_nbt!($variant, $type, $as, $as_mut);
        )*

        impl Debug for NBT {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(NBT::$variant(value) => write!(f, "{:?}", value),)*
                }
            }
        }

        impl Tag {
            /// Converts the provided raw byte into the Tag and returns it
            pub fn from_byte(byte: u8) -> Option<Self> {
                match byte {
                    0 => Some(Tag::End),
                    1 => Some(Tag::Byte),
                    2 => Some(Tag::Short),
                    3 => Some(Tag::Int),
                    4 => Some(Tag::Long),
                    5 => Some(Tag::Float),
                    6 => Some(Tag::Double),
                    7 => Some(Tag::ByteArray),
                    8 => Some(Tag::String),
                    9 => Some(Tag::List),
                    10 => Some(Tag::Compound),
                    11 => Some(Tag::IntArray),
                    12 => Some(Tag::LongArray),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! impl_nbt {
    ($variant:ident, $type:ty, $as:ident, $as_mut:ident) => {
        impl NBT {
            /// Returns a reference to the specified object type
            pub fn $as(&self) -> &$type {
                match self {
                    NBT::$variant(val) => val,
                    _ => panic!("Cannot convert NBT object to inner type"),
                }
            }

            /// Returns a mutable reference to the specified object type
            pub fn $as_mut(&mut self) -> &mut $type {
                match self {
                    NBT::$variant(val) => val,
                    _ => panic!("Cannot convert NBT object to inner type"),
                }
            }
        }

        impl From<$type> for NBT {
            fn from(value: $type) -> Self {
                NBT::$variant(value)
            }
        }

        impl From<NBT> for $type {
            fn from(value: NBT) -> $type {
                match value {
                    NBT::$variant(val) => val,
                    _ => panic!("Cannot convert NBT object to inner type"),
                }
            }
        }
    };
}

generate_nbt!(
    Byte => i8, as_byte, as_mut_byte,
    Short => i16, as_short, as_mut_short,
    Int => i32, as_int, as_mut_int,
    Long => i64, as_long, as_mut_long,
    Float => f32, as_float, as_mut_float,
    Double => f64, as_double, as_mut_double,
    ByteArray => Vec<i8>, as_byte_array, as_mut_byte_array,
    String => String, as_string, as_mut_string,
    List => List, as_list, as_mut_list,
    Compound => Compound, as_compound, as_mut_compound,
    IntArray => Vec<i32>, as_int_array, as_mut_int_array,
    LongArray => Vec<i64>, as_long_array, as_mut_long_array
);
