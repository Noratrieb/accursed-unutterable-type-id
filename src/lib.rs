/// A type that can be identified by a unique `AccursedUnutterableTypeId`.
///
/// # Safety
/// This trait is only allowed to be implemented by the derive macro.
pub unsafe trait AccursedUnutterablyTypeIdentified: 'static {
    fn type_id() -> AccursedUnutterableTypeId;
}

pub struct AccursedUnutterableTypeId(u64);

impl AccursedUnutterableTypeId {
    pub fn of<T>() -> Self
    where
        T: AccursedUnutterablyTypeIdentified,
    {
        T::type_id()
    }
}
