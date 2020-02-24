use coi::{container, Inject};
use std::sync::Arc;

#[derive(Inject)]
#[coi(provides Dep1 with Dep1)]
struct Dep1;

#[derive(Inject)]
#[coi(provides Impl1 with Impl1(dep1))]
struct Impl1(#[coi(inject = "dep1")] Arc<Dep1>);

#[test]
fn main() {
    let container = container! {
        dep1 => Dep1Provider,
        impl1 => Impl1Provider,
    };
    let impl1 = container.resolve::<Impl1>("impl1").expect("Should exist");
    let _dep1: Arc<Dep1> = Arc::clone(&impl1.0);
}
