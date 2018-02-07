//! Crate for safe conversion between units of memory.

#![deny(missing_docs)]
#![no_std]

use core::mem;
use core::ops;

/// [Memory page][memory page] size in bytes.
/// 
/// [memory page]: https://en.wikipedia.org/wiki/Page_(computer_memory)
#[cfg(target_arch = "wasm32")]
pub const PAGE_SIZE: Bytes = Bytes(65536);

/// [Memory page][memory page] size in bytes.
/// 
/// [memory page]: https://en.wikipedia.org/wiki/Page_(computer_memory)
#[cfg(not(target_arch = "wasm32"))]
pub const PAGE_SIZE: Bytes = Bytes(4096);


/// Returns the size of a type in [`Bytes`].
/// 
/// # Example 
/// 
/// ```rust
/// # use memory_units::*;
/// #[repr(C)]
/// struct Hello {
///     a: u32,
///     b: u32,
/// }
/// 
/// assert_eq!(size_of::<Hello>(), Bytes(4 + 4));
/// ```
/// 
/// [`Bytes`]: struct.Bytes.html
#[inline]
pub fn size_of<T>() -> Bytes {
    Bytes(mem::size_of::<T>())
}

/// A trait defining round up conversion between various memory units.
/// 
/// # Example
/// 
/// ```rust
/// # use memory_units::*;
/// // `bytes` contains the size of 1 memory page in bytes.
/// let mut bytes: Bytes = Pages(1).into();
///
/// // Adding 1 to `bytes` makes it larger than the single page.
/// bytes.0 += 1;
/// let pages: Pages = bytes.round_up_to();
/// assert_eq!(pages, Pages(2));
/// ```
pub trait RoundUpTo<T> {
    /// Returns minimum number of `T` to fit amount of space occupied by `self`.
    fn round_up_to(self) -> T;
}

macro_rules! impl_unit_ops {
    ( $name:ident ) => {
        impl<T: Into<Self>> ops::Add<T> for $name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: T) -> Self {
                $name(self.0 + rhs.into().0)
            }
        }

        impl<T: Into<Self>> ops::Sub<T> for $name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: T) -> Self {
                $name(self.0 - rhs.into().0)
            }
        }

        impl<T: Into<Self>> ops::Mul<T> for $name {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: T) -> Self {
                $name(self.0 * rhs.into().0)
            }
        }

        impl<T: Into<Self>> ops::Div<T> for $name {
            type Output = Self;

            #[inline]
            fn div(self, rhs: T) -> Self {
                $name(self.0 / rhs.into().0)
            }
        }
    }
}

/// Memory size specified in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(pub usize);
impl_unit_ops!(Bytes);

/// Memory size specified in words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Words(pub usize);
impl_unit_ops!(Words);

/// Memory size specified in [memory page].
/// 
/// [memory page]: https://en.wikipedia.org/wiki/Page_(computer_memory)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pages(pub usize);
impl_unit_ops!(Pages);

impl From<Words> for Bytes {
    #[inline]
    fn from(words: Words) -> Bytes {
        Bytes(words.0 * mem::size_of::<usize>())
    }
}

#[inline]
fn round_up_to(n: usize, divisor: usize) -> usize {
    (n + divisor - 1) / divisor
}

impl From<Pages> for Bytes {
    #[inline]
    fn from(pages: Pages) -> Bytes {
        Bytes(pages.0 * PAGE_SIZE.0)
    }
}

impl RoundUpTo<Words> for Bytes {
    #[inline]
    fn round_up_to(self) -> Words {
        Words(round_up_to(self.0, mem::size_of::<usize>()))
    }
}

impl RoundUpTo<Pages> for Bytes {
    #[inline]
    fn round_up_to(self) -> Pages {
        Pages(round_up_to(self.0, PAGE_SIZE.0))
    }
}

impl From<Pages> for Words {
    #[inline]
    fn from(pages: Pages) -> Words {
        Words(pages.0 * PAGE_SIZE.0 / mem::size_of::<usize>())
    }
}

impl RoundUpTo<Pages> for Words {
    #[inline]
    fn round_up_to(self) -> Pages {
        let bytes: Bytes = self.into();
        bytes.round_up_to()
    }
}
