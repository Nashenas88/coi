use coi::{ContainerBuilder, Inject};
use std::sync::Arc;

trait Interface1: Inject {
    fn describe(&self) -> &'static str;
}

#[derive(Inject)]
#[provides(Interface1 with Impl1::new)]
struct Impl1;

impl Impl1 {
    fn new() -> Self {
        Impl1
    }
}

impl Interface1 for Impl1 {
    fn describe(&self) -> &'static str {
        "I'm impl1!"
    }
}

trait Interface2: Inject {
    fn deep_describe(&self) -> String;
}

#[derive(Inject)]
#[provides(Interface2 with Impl2::new)]
struct Impl2 {
    #[inject]
    interface1: Arc<dyn Interface1>,
}

impl Impl2 {
    fn new(interface1: Arc<dyn Interface1>) -> Self {
        Self { interface1 }
    }
}

impl Interface2 for Impl2 {
    fn deep_describe(&self) -> String {
        format!("I'm impl2! and I have {}", self.interface1.describe())
    }
}

#[async_std::main]
async fn main() {
    let container = ContainerBuilder::new()
        .register("interface1", Impl1Provider)
        .register("interface2", Impl2Provider)
        .build();
    let interface2 = container
        .resolve::<Arc<dyn Interface2>>("interface2")
        .await
        .expect("Should exist");
    println!("Deep description: {}", interface2.deep_describe());
}
