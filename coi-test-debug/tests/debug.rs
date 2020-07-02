use coi::{coi, container};
use std::sync::Arc;

trait Trait1 {}
#[coi(provides dyn Trait1 + Send + Sync with Impl1)]
struct Impl1;
impl Trait1 for Impl1 {}

trait Trait2 {}
#[coi(provides dyn Trait2 + Send + Sync with Impl2)]
struct Impl2;
impl Trait2 for Impl2 {}

trait Trait3 {}
#[coi(provides dyn Trait3 + Send + Sync with Impl3)]
struct Impl3;
impl Trait3 for Impl3 {}

trait Trait4 {}
#[coi(provides dyn Trait4 + Send + Sync with Impl4)]
struct Impl4;
impl Trait4 for Impl4 {}

trait Trait5 {}

#[allow(unused)]
#[coi(provides dyn Trait5 + Send + Sync with Impl5::new(trait1, trait2, trait3, trait4))]
struct Impl5 {
    #[coi(inject)]
    trait1: Arc<dyn Trait1 + Send + Sync>,
    #[coi(inject)]
    trait2: Arc<dyn Trait2 + Send + Sync>,
    #[coi(inject)]
    trait3: Arc<dyn Trait3 + Send + Sync>,
    #[coi(inject)]
    trait4: Arc<dyn Trait4 + Send + Sync>,
}

impl Impl5 {
    fn new(
        trait1: Arc<dyn Trait1 + Send + Sync>,
        trait2: Arc<dyn Trait2 + Send + Sync>,
        trait3: Arc<dyn Trait3 + Send + Sync>,
        trait4: Arc<dyn Trait4 + Send + Sync>,
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
fn run() {
    let container = container! {
        trait1 => Impl1Provider,
        trait2 => Impl2Provider; scoped,
        trait3 => Impl3Provider,
        trait4 => Impl4Provider; singleton,
        trait5 => Impl5Provider,
    };
    let debugged = format!("{:?}", container);
    assert!(debugged.contains(r#""trait1": []"#));
    assert!(debugged.contains(r#""trait2": []"#));
    assert!(debugged.contains(r#""trait3": []"#));
    assert!(debugged.contains(r#""trait4": []"#));
    assert!(debugged.contains(r#""trait5": ["trait1", "trait2", "trait3", "trait4"]"#));
}
