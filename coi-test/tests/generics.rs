use coi::{container, Inject};

#[derive(Inject)]
#[coi(provides Impl1<T> with Impl1::<T>::new())]
struct Impl1<T>(T)
where
    T: Default;

impl<T> Impl1<T>
where
    T: Default,
{
    fn new() -> Self {
        Self(Default::default())
    }
}

#[test]
fn main() {
    let impl1_provider = Impl1Provider::<bool>::new();
    let container = container! {
        impl1 => impl1_provider,
    };
    let _bool_impl = container
        .resolve::<Impl1<bool>>("impl1")
        .expect("Should exist");
}
