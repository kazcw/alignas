#![no_std]
use core::cmp::{Ord, Ordering, PartialOrd};
use core::fmt::{Debug, Display, Formatter, Result};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

/// Wrap an object of type T to give it the alignment requirements of an object of type A.
///
/// This has a similar effect to #[repr(align(...))], but is parameterized to support the creation
/// of structures that are generic over alignment.
///
/// Example:
/// ```
/// extern crate alignas;
/// use alignas::AlignAs;
/// use std::{mem, slice};
///
/// // put some byte data into the buffer
/// let mut buffer: AlignAs<_, u64> = AlignAs::new([0u8; 64]);
/// buffer[3..18].copy_from_slice(b"some input here");
///
/// // now do something with it that requires aligned access...
/// let mut ints = [0u64; 8];
/// let ptr = &buffer as *const _ as *const u64;
/// ints.copy_from_slice(unsafe { slice::from_raw_parts(ptr, 8) });
/// ```
#[repr(C)]
#[derive(Copy, Clone)]
pub union AlignAs<T: Copy, A: Copy> {
    t: T,
    _marker: A,
}

impl<T: Copy, A: Copy> AlignAs<T, A> {
    /// Put the given T object into an aligned location
    #[inline]
    pub fn new(t: T) -> Self {
        AlignAs { t }
    }
}

impl<T: Copy, A: Copy> Deref for AlignAs<T, A> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.t }
    }
}

impl<T: Copy, A: Copy> DerefMut for AlignAs<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.t }
    }
}

impl<T: Default + Copy, A: Copy> Default for AlignAs<T, A> {
    #[inline]
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Debug + Copy, A: Copy> Debug for AlignAs<T, A> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.deref().fmt(f)
    }
}

impl<T: Display + Copy, A: Copy> Display for AlignAs<T, A> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.deref().fmt(f)
    }
}

impl<T: PartialEq + Copy, A: Copy> PartialEq for AlignAs<T, A> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.deref().eq(rhs)
    }
}
impl<T: Eq + Copy, A: Copy> Eq for AlignAs<T, A> {}

impl<T: PartialOrd + Copy, A: Copy> PartialOrd for AlignAs<T, A> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.deref().partial_cmp(rhs)
    }
}
impl<T: Ord + Copy, A: Copy> Ord for AlignAs<T, A> {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.deref().cmp(rhs)
    }
}

impl<T: Hash + Copy, A: Copy> Hash for AlignAs<T, A> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.deref().hash(h)
    }
}
