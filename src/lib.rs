use std::fmt::{Debug, Formatter};

pub use accursed_unutterable_type_id_derive::AccursedUnutterablyTypeIdentified;

/// A type that can be identified by a unique `AccursedUnutterableTypeId`.
///
/// # Safety
/// This trait is only allowed to be implemented by the derive macro.
pub unsafe trait AccursedUnutterablyTypeIdentified: 'static {
    fn type_id() -> AccursedUnutterableTypeId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccursedUnutterableTypeId(#[doc(hidden)] pub InternalAccursedUnutterableTypeId);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[doc(hidden)]
#[cfg(debug_assertions)]
// we do a little trolling
pub struct InternalAccursedUnutterableTypeId(u128, u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[doc(hidden)]
#[cfg(not(debug_assertions))]
pub struct InternalAccursedUnutterableTypeId(pub u64);

#[cfg(debug_assertions)]
impl InternalAccursedUnutterableTypeId {
    pub fn new(n: u64) -> Self {
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
    pub fn of<T>() -> Self
    where
        T: AccursedUnutterablyTypeIdentified,
    {
        T::type_id()
    }
}
