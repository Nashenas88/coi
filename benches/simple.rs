use coi::{coi, container};
use criterion::Criterion;

trait I {}
#[coi(provides dyn I + Send + Sync with S)]
struct S;

impl I for S {}

pub fn a_simple_resolve(c: &mut Criterion) {
    let container = container! {
        s => SProvider,
    };
    c.bench_function("a simple resolver", |b| {
        b.iter(|| container.resolve::<dyn I + Send + Sync>("s").unwrap());
    });
}
