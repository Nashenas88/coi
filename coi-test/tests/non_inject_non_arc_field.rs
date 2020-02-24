use coi::{container, Inject};

#[derive(Inject)]
#[coi(provides Impl1 with Impl1::new())]
struct Impl1 {
    num: usize,
}

impl Impl1 {
    fn new() -> Self {
        Self { num: 0 }
    }
}

#[test]
fn can_inject_struct_with_non_inject_non_arc_field() {
    let container = container! {
        impl1 => Impl1Provider,
    };
    let impl1 = container.resolve::<Impl1>("impl1").expect("Should exist");
    let _ = impl1.num;
}
