use coi::{coi, container};
use std::sync::Arc;

pub trait Trait1 {
    fn describe(&self) -> &'static str;
}

#[coi(provides dyn Trait1 + Send + Sync with Impl1)]
struct Impl1;

impl Trait1 for Impl1 {
    fn describe(&self) -> &'static str {
        "I'm impl1!"
    }
}

pub trait Trait2 {
    fn deep_describe(&self) -> String;
}

#[coi(provides dyn Trait2 + Send + Sync with Impl2::new(trait1))]
struct Impl2 {
    #[coi(inject)]
    trait1: Arc<dyn Trait1 + Send + Sync>,
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

#[coi(provides JustAStruct with JustAStruct)]
#[derive(Debug)]
pub struct JustAStruct;

#[test]
fn main_test() {
    let container = container! {
        trait1 => Impl1Provider,
        trait2 => Impl2Provider,
        a_struct => JustAStructProvider
    };
    let trait2 = container
        .resolve::<dyn Trait2 + Send + Sync>("trait2")
        .expect("Should exist");
    println!("Deep description: {}", trait2.as_ref().deep_describe());
    let a_struct = container
        .resolve::<JustAStruct>("a_struct")
        .expect("Should exist");
    println!("Got struct! {:?}", a_struct);
}

#[test]
fn can_send_through_threads() {
    let container = container! {
        trait1 => Impl1Provider,
    };
    let _trait1 = container
        .resolve::<dyn Trait1 + Send + Sync>("trait1")
        .expect("Should exist");
    let container = Arc::new(container);
    let thread_container = container.clone();
    let thread = std::thread::spawn(move || {
        let _trait1 = thread_container.resolve::<dyn Trait1 + Send + Sync>("trait1");
    });
    thread.join().expect("Couldn't join thread");
}
