use accursed_unutterable_type_id::{AccursedUnutterableTypeId, AccursedUnutterablyTypeIdentified};

#[derive(AccursedUnutterablyTypeIdentified)]
struct Uwu;

#[derive(AccursedUnutterablyTypeIdentified)]
struct Owo;

fn main() {
    let uwu_id = AccursedUnutterableTypeId::of::<Uwu>();
    let owo_id = AccursedUnutterableTypeId::of::<Owo>();

    assert_ne!(uwu_id, owo_id);
}
