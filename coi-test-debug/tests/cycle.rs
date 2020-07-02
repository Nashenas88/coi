use coi::{coi, container, AnalysisError};
use std::sync::Arc;

trait Trait1 {}
trait Trait2 {}
trait Trait3 {}

#[allow(dead_code)]
#[coi(provides dyn Trait1 + Send + Sync with Impl1::new(t2))]
struct Impl1 {
    #[coi(inject)]
    t2: Arc<dyn Trait2 + Send + Sync>,
}

impl Trait1 for Impl1 {}
impl Impl1 {
    fn new(t2: Arc<dyn Trait2 + Send + Sync>) -> Self {
        Self { t2 }
    }
}

#[allow(dead_code)]
#[coi(provides dyn Trait2 + Send + Sync with Impl2::new(t3))]
struct Impl2 {
    #[coi(inject)]
    t3: Arc<dyn Trait3 + Send + Sync>,
}

impl Trait2 for Impl2 {}
impl Impl2 {
    fn new(t3: Arc<dyn Trait3 + Send + Sync>) -> Self {
        Self { t3 }
    }
}

#[allow(dead_code)]
#[coi(provides dyn Trait3 + Send + Sync with Impl3::new(t1))]
struct Impl3 {
    #[coi(inject)]
    t1: Arc<dyn Trait1 + Send + Sync>,
}

impl Trait3 for Impl3 {}
impl Impl3 {
    fn new(t1: Arc<dyn Trait1 + Send + Sync>) -> Self {
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
        AnalysisError::Cycle(node) => *node == "t1" || *node == "t2" || *node == "t3",
        _ => false,
    }));
}
