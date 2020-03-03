use coi::{container, Container, Inject};
use std::sync::Arc;

pub trait Trait1: Inject {}

pub trait Trait2: Inject {}

#[allow(unused)]
#[derive(Inject)]
struct Impl {
    trait2: Arc<dyn Trait2>,
    data: String,
}

impl Impl {
    fn new(trait2: Arc<dyn Trait2>, data: String) -> Self{
        Self {
            trait2,
            data
        }
    }
}

impl Trait1 for Impl {}

#[derive(Inject)]
#[coi(provides dyn Trait2 with Impl2)]
struct Impl2;

impl Trait2 for Impl2 {}

#[test]
fn main() {
    let x = String::from("3");
    let container = container! {
        trait2 => Impl2Provider,
        trait1 => move |container: &Container| -> coi::Result<Arc<dyn Trait1>> {
            let trait2 = container.resolve::<dyn Trait2>("trait2")?;
            Ok(Arc::new(Impl::new(trait2, x.clone())) as Arc<dyn Trait1>)
        }; scoped,
    };
    let _trait1 = container
        .resolve::<dyn Trait1>("trait1")
        .expect("Trait1 should exist");
    let _trait2 = container
        .resolve::<dyn Trait2>("trait2")
        .expect("Trait2 should exist");
}