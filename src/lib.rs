//! An accursed, unutterable type id.
//!
//! Once upon a time, back when time may not have been a human concept but only a vague idea among the
//! wise, there was [`std::any::TypeId`]

use std::fmt::{Debug, Formatter};

pub use accursed_unutterable_type_id_derive::AccursedUnutterablyTypeIdentified;

/// A type that can be identified by a unique `AccursedUnutterableTypeId`.
///
/// # Safety
/// This trait is only allowed to be implemented by the derive macro.
pub unsafe trait AccursedUnutterablyTypeIdentified: 'static {
    /// Returns the accursed unutterable type id for the type.
    ///
    /// It's suggested to use [`AccursedUnutterableTypeId::of`] instead.
    fn type_id() -> AccursedUnutterableTypeId;
}

/// A unique type id for a type. A fancier (less fancy) [`std::any::TypeId]`] without any
/// internal compiler magic!
///
/// It can easily be derived for your type. The derive is the only way to implement this trait.
/// ```
/// use accursed_unutterable_type_id::{AccursedUnutterableTypeId, AccursedUnutterablyTypeIdentified};
///
/// #[derive(AccursedUnutterablyTypeIdentified)]
/// struct Uwu;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccursedUnutterableTypeId(#[doc(hidden)] InternalAccursedUnutterableTypeId);

impl AccursedUnutterableTypeId {
    #[doc(hidden)]
    pub fn __internal_new(inner: InternalAccursedUnutterableTypeId) -> Self {
        Self(inner)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[doc(hidden)]
#[cfg(debug_assertions)]
// we do a little trolling
pub struct InternalAccursedUnutterableTypeId(u128, u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[doc(hidden)]
#[cfg(not(debug_assertions))]
pub struct InternalAccursedUnutterableTypeId(u64);

#[cfg(debug_assertions)]
impl InternalAccursedUnutterableTypeId {
    pub fn __internal_new(n: u64) -> Self {
        Self(0, n)
    }
    fn inner(self) -> u64 {
        self.1
    }
}

#[cfg(not(debug_assertions))]
impl InternalAccursedUnutterableTypeId {
    pub fn new(n: u64) -> Self {
        Self(n)
    }
    fn inner(self) -> u64 {
        self.0
    }
}

impl Debug for InternalAccursedUnutterableTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner(), f)
    }
}

impl AccursedUnutterableTypeId {
    /// Returns the accursed unutterable type id for the type.
    ///
    /// ```
    /// use accursed_unutterable_type_id::{AccursedUnutterableTypeId, AccursedUnutterablyTypeIdentified};
    ///
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct Uwu;
    ///
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct Owo;
    ///
    /// let uwu_id = AccursedUnutterableTypeId::of::<Uwu>();
    /// let owo_id = AccursedUnutterableTypeId::of::<Owo>();
    ///
    /// assert_ne!(uwu_id, owo_id);
    /// ```
    pub fn of<T>() -> Self
    where
        T: AccursedUnutterablyTypeIdentified,
    {
        T::type_id()
    }
}

mod __doctest {

    /// ```
    /// use accursed_unutterable_type_id::AccursedUnutterablyTypeIdentified;
    ///
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct Uwu<'a, T, const N: usize> {
    ///     _x: &'a [T; N],
    /// }
    /// ```
    mod complex {}
}
