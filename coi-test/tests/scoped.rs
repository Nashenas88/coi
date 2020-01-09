use coi::{container, Inject};
use std::{ops::Deref, sync::Arc};

trait Dep1: Inject {}
trait Dep2: Inject {}

#[derive(Inject)]
#[provides(Impl0 with Impl0)]
struct Impl0;

#[derive(Inject)]
#[provides(dyn Dep1 with Impl1)]
struct Impl1;

#[allow(dead_code)]
#[derive(Inject)]
#[provides(dyn Dep2 with Impl2::new(dep1))]
struct Impl2 {
    #[inject]
    dep1: Arc<dyn Dep1>,
}

impl Impl2 {
    fn new(dep1: Arc<dyn Dep1>) -> Self {
        Self { dep1 }
    }
}

impl Dep1 for Impl1 {}
impl Dep2 for Impl2 {}

#[test]
fn transient_registration_always_returns_new_instance() {
    let mut container = container! {
        dep0 => Impl0Provider
    };

    let dep0_0 = container.resolve::<Impl0>("dep0").unwrap();
    let dep0_1 = container.resolve::<Impl0>("dep0").unwrap();
    assert_ne!(
        dep0_0.deref() as &Impl0 as *const _,
        dep0_1.deref() as &Impl0 as *const _
    );
}

#[test]
fn singleton_registration_always_returns_same_instance_even_when_scoped() {
    let mut container = container! {
        dep1 => Impl1Provider.singleton
    };

    let dep1_0 = container.resolve::<dyn Dep1>("dep1").unwrap();
    let dep1_1 = container.resolve::<dyn Dep1>("dep1").unwrap();
    assert_eq!(
        dep1_0.deref() as &dyn Dep1 as *const _,
        dep1_1.deref() as &dyn Dep1 as *const _
    );
    {
        let container = container.scopable();
        let mut scoped_container = container.scoped();
        let dep1_2 = scoped_container.resolve::<dyn Dep1>("dep1").unwrap();
        assert_eq!(
            dep1_0.deref() as &dyn Dep1 as *const _,
            dep1_1.deref() as &dyn Dep1 as *const _
        );
        assert_eq!(
            dep1_1.deref() as &dyn Dep1 as *const _,
            dep1_2.deref() as &dyn Dep1 as *const _
        );
    }
}

#[test]
fn scoped_registration_always_returns_same_instance_within_same_scope() {
    let mut container = container! {
        dep1 => Impl1Provider.singleton,
        dep2 => Impl2Provider.scoped
    };

    let dep2_0 = container.resolve::<dyn Dep2>("dep2").unwrap();
    let dep2_1 = container.resolve::<dyn Dep2>("dep2").unwrap();
    assert_eq!(
        dep2_0.deref() as &dyn Dep2 as *const _,
        dep2_1.deref() as &dyn Dep2 as *const _
    );
    {
        let container = container.scopable();
        let mut scoped_container = container.scoped();
        let dep2_2 = scoped_container.resolve::<dyn Dep2>("dep2").unwrap();
        let dep2_3 = scoped_container.resolve::<dyn Dep2>("dep2").unwrap();
        assert_ne!(
            dep2_0.deref() as &dyn Dep2 as *const _,
            dep2_2.deref() as &dyn Dep2 as *const _
        );
        assert_ne!(
            dep2_1.deref() as &dyn Dep2 as *const _,
            dep2_2.deref() as &dyn Dep2 as *const _
        );
        assert_ne!(
            dep2_0.deref() as &dyn Dep2 as *const _,
            dep2_3.deref() as &dyn Dep2 as *const _
        );
        assert_ne!(
            dep2_1.deref() as &dyn Dep2 as *const _,
            dep2_3.deref() as &dyn Dep2 as *const _
        );
        assert_eq!(
            dep2_2.deref() as &dyn Dep2 as *const _,
            dep2_3.deref() as &dyn Dep2 as *const _
        );
    }
}
