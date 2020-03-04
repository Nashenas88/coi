use coi::{container, Inject};

pub trait Trait1: Inject {
    fn a(&self) -> &'static str;
}

pub trait Trait2: Inject {
    fn b(&self) -> &'static str;
}

#[derive(Inject)]
#[coi(provides dyn Trait1 as ImplTrait1Provider with Impl)]
#[coi(provides dyn Trait2 as ImplTrait2Provider with Impl)]
struct Impl;

impl Trait1 for Impl {
    fn a(&self) -> &'static str {
        "a"
    }
}

impl Trait2 for Impl {
    fn b(&self) -> &'static str {
        "b"
    }
}
#[test]
fn main() {
    let container = container! {
        trait1 => ImplTrait1Provider,
        trait2 => ImplTrait2Provider,
    };
    let _trait1 = container
        .resolve::<dyn Trait1>("trait1")
        .expect("Trait1 should exist");
    let _trait2 = container
        .resolve::<dyn Trait2>("trait2")
        .expect("Trait2 should exist");
}
