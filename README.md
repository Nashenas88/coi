# coi

[![Build Status](https://travis-ci.org/Nashenas88/coi.svg?branch=master)](https://travis-ci.org/Nashenas88/coi)
[![docs.rs](https://docs.rs/coi/badge.svg)](https://docs.rs/coi)
[![crates.io](https://img.shields.io/crates/v/coi.svg)](https://crates.io/crates/coi)

Dependency Injection in Rust

The goal of this crate is to provide a simple dependency injection framework
that is easy to use. Performance is not an initial concern, but might be later
on as the crate matures.

### Example 

```rust
use coi::{container, Inject};
use std::sync::Arc;

// The trait you'd like to inject
pub trait Trait1 {
    fn describe(&self) -> &'static str;
}

// derive `Inject` on all structs that will provide the implementation
#[derive(Inject)]
#[coi(provides dyn Trait1 with Impl1)]
struct Impl1;

// actually impl the trait
impl Trait1 for Impl1 {
    fn describe(&self) -> &'static str {
        "I'm impl1!"
    }
}

pub trait Trait2 {
    fn deep_describe(&self) -> String;
}

#[derive(Inject)]
#[coi(provides dyn Trait2 with Impl2::new(trait1))]
struct Impl2 {
    // inject dependencies by Arc<dyn SomeTrait>
    #[coi(inject)]
    trait1: Arc<dyn Trait1 + Send + Sync + 'static>,
}

impl Impl2 {
    fn new(trait1: Arc<dyn Trait1 + Send + Sync + 'static>) -> Self {
        Self { trait1 }
    }
}

impl Trait2 for Impl2 {
    fn deep_describe(&self) -> String {
        format!("I'm impl2! and I have {}", self.trait1.describe())
    }
}

// It even works on structs
#[derive(Debug, Inject)]
#[coi(provides JustAStruct with JustAStruct)]
pub struct JustAStruct;

fn main() {
    // Then construct your container with the helper `container!` macro
    let container = container!{
        trait1 => Impl1Provider,
        trait2 => Impl2Provider; scoped,
        struct => JustAStructProvider; singleton
    };

    // And resolve away!
    let trait2 = container
        .resolve::<dyn Trait2 + Send + Sync + 'static>("trait2")
        .expect("Should exist");
    println!("Deep description: {}", trait2.as_ref().deep_describe());
    let a_struct = container
        .resolve::<JustAStruct>("struct")
        .expect("Should exist");
    println!("Got struct! {:?}", a_struct);
}
```

#### Name

The name coi comes from an inversion of the initialism IoC (Inversion of
Control).

#### License

<sup>
Licensed under either of <a href="LICENSE.Apache-2.0">Apache License, Version
2.0</a> or <a href="LICENSE.MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

`SPDX-License-Identifier: MIT OR Apache-2.0`
