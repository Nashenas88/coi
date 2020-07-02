use coi::{coi, container, provide_closure};
use std::sync::Arc;

pub trait Trait1 {}

pub trait Trait2 {}

#[allow(unused)]
struct Impl {
    trait2: Arc<dyn Trait2 + Send + Sync>,
    data: String,
}

impl Impl {
    fn new(trait2: Arc<dyn Trait2 + Send + Sync>, data: String) -> Self {
        Self { trait2, data }
    }
}

impl Trait1 for Impl {}

#[coi(provides dyn Trait2 + Send + Sync with Impl2)]
struct Impl2;

impl Trait2 for Impl2 {}

#[test]
fn run() {
    let x = String::from("3");
    let trait1_provider = provide_closure!(
        move |trait2: Arc<dyn Trait2 + Send + Sync>| -> coi::Result<Arc<dyn Trait1 + Send + Sync>> {
            Ok(Arc::new(Impl::new(trait2, x.clone())) as Arc<dyn Trait1 + Send + Sync>)
        }
    );
    let container = container! {
        trait2 => Impl2Provider,
        trait1 => trait1_provider; scoped,
    };
    let _trait1 = container
        .resolve::<dyn Trait1 + Send + Sync>("trait1")
        .expect("Trait1 should exist");
    let _trait2 = container
        .resolve::<dyn Trait2 + Send + Sync>("trait2")
        .expect("Trait2 should exist");
}
