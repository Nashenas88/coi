#![allow(soft_unstable)]
#![feature(test)]
extern crate test;

use coi::{coi, container};
use test::Bencher;

trait I {}
#[coi(provides dyn I + Send + Sync with S)]
struct S;

impl I for S {}

#[bench]
fn a_simple_resolve(b: &mut Bencher) {

    let container = container! {
        s => SProvider,
    };
    b.iter(|| container.resolve::<dyn I + Send + Sync>("s").unwrap());
}