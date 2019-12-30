use coi::{ContainerBuilder, Inject};
use std::sync::Arc;

trait Trait1: Inject {
    fn describe(&self) -> &'static str;
}

#[derive(Inject)]
#[provides(Trait1 with Impl1::new)]
struct Impl1;

impl Impl1 {
    fn new() -> Self {
        Impl1
    }
}

impl Trait1 for Impl1 {
    fn describe(&self) -> &'static str {
        "I'm impl1!"
    }
}

trait Trait2: Inject {
    fn deep_describe(&self) -> String;
}

#[derive(Inject)]
#[provides(Trait2 with Impl2::new)]
struct Impl2 {
    #[inject]
    trait1: Arc<dyn Trait1>,
}

impl Impl2 {
    fn new(trait1: Arc<dyn Trait1>) -> Self {
        Self { trait1 }
    }
}

impl Trait2 for Impl2 {
    fn deep_describe(&self) -> String {
        format!("I'm impl2! and I have {}", self.trait1.describe())
    }
}

#[async_std::main]
async fn main() {
    let container = ContainerBuilder::new()
        .register("trait1", Impl1Provider)
        .register("trait2", Impl2Provider)
        .build();
    let trait2 = container
        .resolve::<Arc<dyn Trait2>>("trait2")
        .await
        .expect("Should exist");
    println!("Deep description: {}", trait2.deep_describe());
}
