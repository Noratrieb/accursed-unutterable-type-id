use accursed_unutterable_type_id::{AccursedUnutterableTypeId, AccursedUnutterablyTypeIdentified};

#[derive(AccursedUnutterablyTypeIdentified)]
struct Uwu;

#[derive(AccursedUnutterablyTypeIdentified)]
struct Owo;

#[derive(AccursedUnutterablyTypeIdentified)]
struct Hi;

#[derive(AccursedUnutterablyTypeIdentified)]
struct OhLord;

fn main() {
    let uwu_id = AccursedUnutterableTypeId::of::<Uwu>();
    let owo_id = AccursedUnutterableTypeId::of::<Owo>();
    let hi_id = AccursedUnutterableTypeId::of::<Hi>();
    let oh_lord_id = AccursedUnutterableTypeId::of::<OhLord>();

    assert_ne!(uwu_id, owo_id);
    assert_ne!(owo_id, hi_id);
    assert_ne!(hi_id, oh_lord_id);
}
