use coi::{container, AnalysisError, Inject};
use std::sync::Arc;

trait Trait1: Inject {}
trait Trait2: Inject {}
trait Trait3: Inject {}

#[allow(dead_code)]
#[derive(Inject)]
#[provides(dyn Trait1 with Impl1::new(t2))]
struct Impl1 {
    #[inject]
    t2: Arc<dyn Trait2>,
}

impl Trait1 for Impl1 {}
impl Impl1 {
    fn new(t2: Arc<dyn Trait2>) -> Self {
        Self { t2 }
    }
}

#[allow(dead_code)]
#[derive(Inject)]
#[provides(dyn Trait2 with Impl2::new(t3))]
struct Impl2 {
    #[inject]
    t3: Arc<dyn Trait3>,
}

impl Trait2 for Impl2 {}
impl Impl2 {
    fn new(t3: Arc<dyn Trait3>) -> Self {
        Self { t3 }
    }
}

#[allow(dead_code)]
#[derive(Inject)]
#[provides(dyn Trait3 with Impl3::new(t1))]
struct Impl3 {
    #[inject]
    t1: Arc<dyn Trait1>,
}

impl Trait3 for Impl3 {}
impl Impl3 {
    fn new(t1: Arc<dyn Trait1>) -> Self {
        Self { t1 }
    }
}

#[test]
fn validate_cycle() {
    let container = container! {
        t1 => Impl1Provider,
        t2 => Impl2Provider,
        t3 => Impl3Provider,
    };

    let res = container.analyze();
    assert!(res.is_err());
    let errors = res.unwrap_err();
    println!("{:?}", errors);
    // unfortunately, the iteration is not deterministic, so we need to match
    // on any of the items that might be in the cycle
    assert!(errors.iter().any(|e| match e {
        AnalysisError::Cycle(node) => node == "t1" || node == "t2" || node == "t3",
        _ => false,
    }));
}
