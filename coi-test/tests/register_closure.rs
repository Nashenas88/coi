use coi::{container, injectable, provide_closure, Inject};
use std::sync::Arc;

pub trait Trait1 {}

pub trait Trait2 {}

#[allow(unused)]
#[derive(Inject)]
struct Impl {
    trait2: Arc<dyn Trait2 + Send + Sync + 'static>,
    data: String,
}

impl Impl {
    fn new(trait2: injectable!(dyn Trait2), data: String) -> Self {
        Self { trait2, data }
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
    let trait1_provider = provide_closure!(
        move |trait2: Arc<dyn Trait2 + Send + Sync + 'static>| -> coi::Result<Arc<dyn Trait1 + Send + Sync + 'static>> {
            Ok(Arc::new(Impl::new(trait2, x.clone())) as injectable!(dyn Trait1))
        }
    );
    let container = container! {
        trait2 => Impl2Provider,
        trait1 => trait1_provider; scoped,
    };
    let _trait1 = container
        .resolve::<dyn Trait1 + Send + Sync + 'static>("trait1")
        .expect("Trait1 should exist");
    let _trait2 = container
        .resolve::<dyn Trait2 + Send + Sync + 'static>("trait2")
        .expect("Trait2 should exist");
}
