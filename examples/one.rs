use accursed_unutterable_type_id::AccursedUnutterablyTypeIdentified;

#[derive(AccursedUnutterablyTypeIdentified)]
struct Uwu<T, const N: usize>
where
    T: 'static,
{
    _x: [T; N],
}

fn main() {}
