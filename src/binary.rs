///
/// The following macro is used to generate the Wrappers for existing generic rust types
/// for purposes of abstracting the way they may be serialized over the wire in the backend.
///

#[macro_export]
macro_rules! generate {
    ($name:ident, $type:ty) => {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct $name {
            val: $type,
        }

        impl $name {
            pub fn new(val: $type) -> Self {
                Self { val }
            }

            pub fn get(self) -> $type {
                self.val
            }
        }

        impl AsRef<$type> for $name {
            fn as_ref(&self) -> &$type {
                &self.val
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.val)
            }
        }

        impl core::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.val
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.val
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> $name {
                Self::new(value)
            }
        }

        impl From<$name> for $type {
            fn from(value: $name) -> $type {
                value.get()
            }
        }
    };
}

generate!(u24, u32);
generate!(w32, u32);
generate!(v32, i32);
generate!(w64, u64);
generate!(v64, i64);
