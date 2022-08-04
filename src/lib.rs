//! An accursed, unutterable type id.
//!
//! Once upon a time, back when time may not have been a human concept but only a vague idea among the
//! wise, there was [`std::any::TypeId`].
//!
//! It was a good type, and many of these early inhabitants of planet earth were quite fond of it.
//! Yet, there was a fundamental issue in it, that even the elders were not able to resolve: It
//! required significant magic from the compiler. The peoples back then were no stranger to magic,
//! but even just the thought of having magic in their type ids caused numerous wars among them.
//!
//! After the last and most brutal of the so called "type-id" wars, one especially clever member of
//! one of the leading clans for type id research had a breakthrough. They found a new method to
//! implement type ids in user code! Even though their method had a significant disadvantage in that
//! it had to be implemented using a derive macro (futuristic technology that the elderly have only
//! dreamt of back then). Yet this change was accepted, and peace among the peoples ensured.
//!
//! Using it is as simple as slapping a derive macro on your type
//! and then getting the type id using [`AccursedUnutterableTypeId::of`].
//!
//! ```
//! use accursed_unutterable_type_id::{AccursedUnutterableTypeId, AccursedUnutterablyTypeIdentified};
//!
//! #[derive(AccursedUnutterablyTypeIdentified)]
//! struct Uwu;
//!
//! let type_id = AccursedUnutterableTypeId::of::<Uwu>();
//! println!("{type_id:?}")
//! ```

use std::{
    fmt::{Debug, Formatter},
    hash::Hash,
};

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

/// A unique type id for a type. A fancier (less fancy) [`std::any::TypeId`] without any
/// internal compiler magic!
///
/// It can easily be derived for your type. The derive is the only way to implement this trait.
/// ```
/// use accursed_unutterable_type_id::AccursedUnutterablyTypeIdentified;
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
pub struct InternalAccursedUnutterableTypeId(u128, u64, u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[doc(hidden)]
#[cfg(not(debug_assertions))]
pub struct InternalAccursedUnutterableTypeId(u64, u64);

#[cfg(debug_assertions)]
impl InternalAccursedUnutterableTypeId {
    pub fn __internal_new(s: u64, g: u64) -> Self {
        Self(0, s, g)
    }
    fn inner(self) -> (u64, u64) {
        (self.1, self.2)
    }
}

#[cfg(not(debug_assertions))]
impl InternalAccursedUnutterableTypeId {
    pub fn __internal_new(s: u64, g: u64) -> Self {
        Self(s, g)
    }
    fn inner(self) -> (u64, u64) {
        (self.0, self.1)
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
    /// struct Uwu<T: 'static, /* const N: usize */> {
    ///     _x: [T; /* N */ 0],
    /// }
    /// ```
    mod complex {}

    /// ```
    /// use accursed_unutterable_type_id::AccursedUnutterablyTypeIdentified;
    ///
    /// trait Sussy {}
    ///
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct Uwu<T: Sussy> {
    ///     _x: T,
    /// }
    /// ```
    mod type_bounds {}

    /// ```
    /// use accursed_unutterable_type_id::AccursedUnutterablyTypeIdentified;
    ///
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct Uwu<T> {
    ///     _x: T,
    /// }
    /// ```
    mod static_ty_param {}

    /// ```
    /// use accursed_unutterable_type_id::AccursedUnutterablyTypeIdentified;
    ///
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct Uwu<T> where T: Copy {
    ///     _x: T,
    /// }
    /// ```
    mod where_clause {}

    /// ```
    /// use accursed_unutterable_type_id::{AccursedUnutterableTypeId as Id, AccursedUnutterablyTypeIdentified};
    ///
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct Uwu<T> {
    ///     _x: T,
    /// }
    ///
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct A;
    /// #[derive(AccursedUnutterablyTypeIdentified)]
    /// struct B;
    ///
    /// assert_ne!(Id::of::<Uwu<A>>(), Id::of::<Uwu<B>>());
    /// ```
    mod type_param {}
}
