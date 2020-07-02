use coi::{container, Container, Provide, Result};
use std::sync::Arc;

pub trait Trait1 {
    fn describe(&self) -> &'static str;
}

struct Impl1;

struct Impl1Provider;

impl Provide for Impl1Provider {
    type Output = dyn Trait1 + Send + Sync;

    fn provide(&self, _: &Container) -> Result<Arc<Self::Output>> {
        Ok(Arc::new(Impl1) as Arc<dyn Trait1 + Send + Sync>)
    }
}

impl Trait1 for Impl1 {
    fn describe(&self) -> &'static str {
        "I'm impl1!"
    }
}

pub trait Trait2 {
    fn deep_describe(&self) -> String;
}

struct Impl2 {
    trait1: Arc<dyn Trait1 + Send + Sync>,
}

struct Impl2Provider;

impl Provide for Impl2Provider {
    type Output = dyn Trait2 + Send + Sync;

    fn provide(&self, container: &Container) -> Result<Arc<Self::Output>> {
        let trait1 = container.resolve::<dyn Trait1 + Send + Sync>("trait1")?;
        Ok(Arc::new(Impl2::new(trait1)) as Arc<dyn Trait2 + Send + Sync>)
    }
}

impl Impl2 {
    fn new(trait1: Arc<dyn Trait1 + Send + Sync>) -> Self {
        Self { trait1 }
    }
}

impl Trait2 for Impl2 {
    fn deep_describe(&self) -> String {
        format!("I'm impl2! and I have {}", self.trait1.describe())
    }
}

pub struct JustAStruct;

struct JustAStructProvider;

impl Provide for JustAStructProvider {
    type Output = JustAStruct;

    fn provide(&self, _: &Container) -> Result<Arc<Self::Output>> {
        Ok(Arc::new(JustAStruct))
    }
}

#[test]
fn main() {
    let container = container! {
        trait1 => Impl1Provider,
        trait2 => Impl2Provider
    };
    let trait2 = container
        .resolve::<dyn Trait2 + Send + Sync>("trait2")
        .expect("Should exist");
    println!("Deep description: {}", trait2.as_ref().deep_describe());
}
