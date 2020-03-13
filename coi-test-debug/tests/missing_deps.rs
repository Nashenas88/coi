use coi::{container, injectable, AnalysisError, Inject};
use std::sync::Arc;

trait Trait1 {}
trait Trait2 {}
trait Trait3 {}

#[allow(dead_code)]
#[derive(Inject)]
#[coi(provides dyn Trait1 with Impl1::new(t3))]
struct Impl1 {
    #[coi(inject)]
    t3: Arc<dyn Trait3 + Send + Sync + 'static>,
}

impl Trait1 for Impl1 {}
impl Impl1 {
    fn new(t3: injectable!(dyn Trait3)) -> Self {
        Self { t3 }
    }
}

#[allow(dead_code)]
#[derive(Inject)]
#[coi(provides dyn Trait2 with Impl2::new(t3))]
struct Impl2 {
    #[coi(inject)]
    t3: Arc<dyn Trait3 + Send + Sync + 'static>,
}

impl Trait2 for Impl2 {}
impl Impl2 {
    fn new(t3: injectable!(dyn Trait3)) -> Self {
        Self { t3 }
    }
}

#[test]
fn validate_missing() {
    let container = container! {
        t1 => Impl1Provider,
        t2 => Impl2Provider,
    };

    let res = container.analyze();
    assert!(res.is_err());
    let errors = res.unwrap_err();
    assert!(errors.iter().any(|e| match e {
        AnalysisError::Missing(from, to) => from == "t1" && to == "t3",
        _ => false,
    }));
    assert!(errors.iter().any(|e| match e {
        AnalysisError::Missing(from, to) => from == "t2" && to == "t3",
        _ => false,
    }));
}
