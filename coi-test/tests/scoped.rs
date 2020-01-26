use coi::{container, Container, Inject, Provide};
use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

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
    let container = Arc::new(Mutex::new(container! {
        dep1 => Impl1Provider.singleton
    }));

    let dep1_0 = container
        .lock()
        .unwrap()
        .resolve::<dyn Dep1>("dep1")
        .unwrap();
    let dep1_1 = container
        .lock()
        .unwrap()
        .resolve::<dyn Dep1>("dep1")
        .unwrap();
    assert_eq!(
        dep1_0.deref() as &dyn Dep1 as *const _,
        dep1_1.deref() as &dyn Dep1 as *const _
    );
    {
        let container = Container::scopable(Arc::clone(&container));
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
    let container = Arc::new(Mutex::new(container! {
        dep1 => Impl1Provider.singleton,
        dep2 => Impl2Provider.scoped
    }));

    let dep2_0 = container
        .lock()
        .unwrap()
        .resolve::<dyn Dep2>("dep2")
        .unwrap();
    let dep2_1 = container
        .lock()
        .unwrap()
        .resolve::<dyn Dep2>("dep2")
        .unwrap();
    assert_eq!(
        dep2_0.deref() as &dyn Dep2 as *const _,
        dep2_1.deref() as &dyn Dep2 as *const _
    );
    {
        let container = Container::scopable(container);
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

trait Id: Inject {
    fn id(&self) -> usize;
}

struct Unique {
    id: usize,
}

impl Inject for Unique {}

impl Id for Unique {
    fn id(&self) -> usize {
        self.id
    }
}

struct UniqueProvider {
    count: AtomicUsize,
}

impl UniqueProvider {
    fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }
}

impl Provide for UniqueProvider {
    type Output = dyn Id;

    fn provide(&self, _: &mut Container) -> coi::Result<Arc<Self::Output>> {
        let count = self.count.fetch_add(1, Ordering::Relaxed);
        Ok(Arc::new(Unique { id: count }) as Arc<dyn Id>)
    }
}

trait Hold: Inject {
    fn get_id(&self) -> usize;
}

#[derive(Inject)]
#[provides(dyn Hold with Holder::new(id))]
struct Holder {
    #[inject]
    id: Arc<dyn Id>,
}

impl Holder {
    fn new(id: Arc<dyn Id>) -> Self {
        Self { id }
    }
}

impl Hold for Holder {
    fn get_id(&self) -> usize {
        self.id.id()
    }
}

trait Dep3: Inject {
    fn get_ids(&self) -> (usize, usize);
}

#[derive(Inject)]
#[provides(dyn Dep3 with Impl3::new(id, hold))]
struct Impl3 {
    #[inject]
    id: Arc<dyn Id>,
    #[inject]
    hold: Arc<dyn Hold>,
}

impl Impl3 {
    fn new(id: Arc<dyn Id>, hold: Arc<dyn Hold>) -> Self {
        Self { id, hold }
    }
}

impl Dep3 for Impl3 {
    fn get_ids(&self) -> (usize, usize) {
        (self.id.id(), self.hold.get_id())
    }
}

#[test]
fn scoped_registration_provides_same_instance_regardless_of_nesting_order() {
    let unique_provider = UniqueProvider::new();
    let container = container! {
        id => unique_provider.scoped,
        hold => HolderProvider.transient,
        dep3 => Impl3Provider.scoped,
    };
    let mut scoped_container = Container::scopable(Arc::new(Mutex::new(container))).scoped();
    let dep3 = scoped_container.resolve::<dyn Dep3>("dep3").unwrap();
    let (id1, id2) = dep3.get_ids();
    assert_eq!(
        id1, id2,
        "If the ids are different, they were resolved in different scopes!"
    );
}
