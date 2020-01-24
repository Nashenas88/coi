use coi::{container, Inject};
use std::sync::Arc;

trait Trait1: Inject {}
#[derive(Inject)]
#[provides(dyn Trait1 with Impl1)]
struct Impl1;
impl Trait1 for Impl1 {}

trait Trait2: Inject {}
#[derive(Inject)]
#[provides(dyn Trait2 with Impl2)]
struct Impl2;
impl Trait2 for Impl2 {}

trait Trait3: Inject {}
#[derive(Inject)]
#[provides(dyn Trait3 with Impl3)]
struct Impl3;
impl Trait3 for Impl3 {}

trait Trait4: Inject {}
#[derive(Inject)]
#[provides(dyn Trait4 with Impl4)]
struct Impl4;
impl Trait4 for Impl4 {}

trait Trait5: Inject {}

#[allow(unused)]
#[derive(Inject)]
#[provides(dyn Trait5 with Impl5::new(trait1, trait2, trait3, trait4))]
struct Impl5 {
    #[inject]
    trait1: Arc<dyn Trait1>,
    #[inject]
    trait2: Arc<dyn Trait2>,
    #[inject]
    trait3: Arc<dyn Trait3>,
    #[inject]
    trait4: Arc<dyn Trait4>,
}

impl Impl5 {
    fn new(
        trait1: Arc<dyn Trait1>,
        trait2: Arc<dyn Trait2>,
        trait3: Arc<dyn Trait3>,
        trait4: Arc<dyn Trait4>,
    ) -> Self {
        Self {
            trait1,
            trait2,
            trait3,
            trait4,
        }
    }
}

impl Trait5 for Impl5 {}

#[test]
fn main() {
    let container = container! {
        trait1 => Impl1Provider,
        trait2 => Impl2Provider.scoped,
        trait3 => Impl3Provider,
        trait4 => Impl4Provider.singleton,
        trait5 => Impl5Provider,
    };
    let debugged = format!("{:?}", container);
    assert!(debugged.contains(r#""trait1": []"#));
    assert!(debugged.contains(r#""trait2": []"#));
    assert!(debugged.contains(r#""trait3": []"#));
    assert!(debugged.contains(r#""trait4": []"#));
    assert!(debugged.contains(r#""trait5": ["trait1", "trait2", "trait3", "trait4"]"#));
}
