use coi::{coi, container};

pub trait Trait1 {
    fn a(&self) -> &'static str;
}

pub trait Trait2 {
    fn b(&self) -> &'static str;
}

#[coi(provides dyn Trait1 + Send + Sync as ImplTrait1Provider with Impl)]
#[coi(provides dyn Trait2 + Send + Sync as ImplTrait2Provider with Impl)]
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
fn run() {
    let container = container! {
        trait1 => ImplTrait1Provider,
        trait2 => ImplTrait2Provider,
    };
    let _trait1 = container
        .resolve::<dyn Trait1 + Send + Sync>("trait1")
        .expect("Trait1 should exist");
    let _trait2 = container
        .resolve::<dyn Trait2 + Send + Sync>("trait2")
        .expect("Trait2 should exist");
}
