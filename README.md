# accursed-unutterable-type-id

An accursed, unutterable type id.

Once upon a time, back when time may not have been a human concept but only a vague idea among the
wise, there was [`std::any::TypeId`].

It was a good type, and many of these early inhabitants of planet earth were quite fond of it.
Yet, there was a fundamental issue in it, that even the elders were not able to resolve: It
required significant magic from the compiler. The peoples back then were no stranger to magic,
but even just the thought of having magic in their type ids caused numerous wars among them.

After the last and most brutal of the so called "type-id" wars, one especially clever member of
one of the leading clans for type id research had a breakthrough. They found a new method to
implement type ids in user code! Even though their method had a significant disadvantage in that
it had to be implemented using a derive macro (futuristic technology that the elderly have only
dreamt of back then). Yet this change was accepted, and peace among the peoples ensured.

Using it is as simple as slapping a derive macro on your type
and then getting the type id using [`AccursedUnutterableTypeId::of`].

```rust
use accursed_unutterable_type_id::{AccursedUnutterableTypeId, AccursedUnutterablyTypeIdentified};

#[derive(AccursedUnutterablyTypeIdentified)]
struct Uwu;

let type_id = AccursedUnutterableTypeId::of::<Uwu>();
println!("{type_id:?}")
```