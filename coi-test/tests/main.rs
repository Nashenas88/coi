use coi::{container, Inject};
use std::sync::Arc;

pub trait Trait1: Inject {
    fn describe(&self) -> &'static str;
}

#[derive(Inject)]
#[coi(provides dyn Trait1 with Impl1)]
struct Impl1;

impl Trait1 for Impl1 {
    fn describe(&self) -> &'static str {
        "I'm impl1!"
    }
}

pub trait Trait2: Inject {
    fn deep_describe(&self) -> String;
}

#[derive(Inject)]
#[coi(provides dyn Trait2 with Impl2::new(trait1))]
struct Impl2 {
    #[coi(inject)]
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

#[derive(Debug, Inject)]
#[coi(provides JustAStruct with JustAStruct)]
pub struct JustAStruct;

#[test]
fn main() {
    let container = container! {
        trait1 => Impl1Provider,
        trait2 => Impl2Provider,
        struct => JustAStructProvider
    };
    let trait2 = container
        .resolve::<dyn Trait2>("trait2")
        .expect("Should exist");
    println!("Deep description: {}", trait2.as_ref().deep_describe());
    let a_struct = container
        .resolve::<JustAStruct>("struct")
        .expect("Should exist");
    println!("Got struct! {:?}", a_struct);
}

#[test]
fn can_send_through_threads() {
    let container = container! {
        trait1 => Impl1Provider,
    };
    let _trait1 = container
        .resolve::<dyn Trait1>("trait1")
        .expect("Should exist");
    let thread_container = container.clone();
    let thread = std::thread::spawn(move || {
        let _trait1 = thread_container.resolve::<dyn Trait1>("trait1");
    });
    thread.join().expect("Couldn't join thread");
}
