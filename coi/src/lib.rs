//! Coi provides an easy to use dependency injection framework.
//! Currently, this crate provides the following:
//! - **[`coi::Inject` (trait)]** - a marker trait that indicates a trait or struct is injectable.
//! - **[`coi::Provide`]** - a trait that indicates a struct is capable of providing a specific
//! implementation of some injectable trait. This is generated for you if you use
//! [`coi::Inject` (derive)], but can also be written manually.
//! - **[`coi::Container`]** - a container to manage the lifetime of all dependencies. This is still
//! in its early stages, and currently only supports objects that are recreated with each request to
//! [`coi::Container::resolve`].
//! - **[`coi::ContainerBuilder`]** - a builder for the above container to simplify construction and
//! guarantee immutability after construction.
//!
//! [`coi::Inject` (trait)]: trait.Inject.html
//! [`coi::Inject` (derive)]: derive.Inject.html
//! [`coi::Provide`]: trait.Provide.html
//! [`coi::Container`]: struct.Container.html
//! [`coi::Container::resolve`]: struct.Container.html#method.resolve
//! [`coi::ContainerBuilder`]: struct.ContainerBuilder.html
//!
//! # Example
//!
//! ```rust
//! use async_std;
//! use coi::{ContainerBuilder, Inject};
//! use std::sync::Arc;
//!
//! // Mark injectable traits by inheriting the `Inject` trait.
//! trait Trait1: Inject {
//!     fn describe(&self) -> &'static str;
//! }
//!
//! // For structs that will provide the implementation of an injectable trait, derive `Inject`
//! // and specify which expr will be used to inject which trait. The method can be any path.
//! // The arguments for the method are derived from fields marked with the attribute `#[inject]`
//! // (See Impl2 below).
//! #[derive(Inject)]
//! // Currently, only one trait can be provided, but this will likely be expanded on in future
//! // versions of this crate.
//! #[provides(dyn Trait1 with Impl1)]
//! struct Impl1;
//!
//! // Don't forget to actually implement the trait ;).
//! impl Trait1 for Impl1 {
//!     fn describe(&self) -> &'static str {
//!         "I'm impl1!"
//!     }
//! }
//!
//! // Mark injectable traits by inheriting the `Inject` trait.
//! trait Trait2: Inject {
//!     fn deep_describe(&self) -> String;
//! }
//!
//! // For structs that will provide the implementation of an injectable trait, derive `Inject`
//! // and specify which method will be used to inject which trait. The arguments for the method
//! // are derived from fields marked with the attribute `#[inject]`, so the parameter name must
//! // match a field name.
//! #[derive(Inject)]
//! #[provides(dyn Trait2 with Impl2::new(trait1))]
//! struct Impl2 {
//!     // The name of the field is important! It must match the name that's registered in the
//!     // container when the container is being built! This is similar to the behavior of
//!     // dependency injection libraries in other languages.
//!     #[inject]
//!     trait1: Arc<dyn Trait1>,
//! }
//!
//! // Implement the provider method
//! impl Impl2 {
//!     // Note: The param name here doesn't actually matter.
//!     fn new(trait1: Arc<dyn Trait1>) -> Self {
//!         Self { trait1 }
//!     }
//! }
//!
//! // Again, don't forget to actually implement the trait ;).
//! impl Trait2 for Impl2 {
//!     fn deep_describe(&self) -> String {
//!         format!("I'm impl2! and I have {}", self.trait1.describe())
//!     }
//! }
//!
//! // You might note that Container::resolve is async. This is to allow any provider to be async
//! // and since we don't know from the resolution perspective whether any provider will need to be
//! // async, they all have to be. This might be configurable through feature flags in a future
//! // version of the library.
//! #[async_std::main]
//! async fn main() {
//!     // "Provider" structs are automatically generated through the `Inject` attribute. They
//!     // append `Provider` to the name of the struct that is being derive (make sure you don't
//!     // any structs with the same name or your code will fail to compile.
//!     // Reminder: Make sure you use the same key here as the field names of the structs that
//!     // require these impls.
//!     let container = ContainerBuilder::new()
//!         .register("trait1", Impl1Provider)
//!         .register("trait2", Impl2Provider)
//!         .build();
//!
//!     // Once the container is built, you can now resolve any particular instance by its key and
//!     // the trait it provides. This crate currently only supports `Arc<dyn Trait>`, but this may
//!     // be expanded in a future version of the crate.
//!     let trait2 = container
//!         // Note: Getting the key wrong will produce an error telling you which key in the
//!         // chain of dependencies caused the failure (future versions might provider a vec of
//!         // chain that lead to the failure). Getting the type wrong will only tell you which key
//!         // had the wrong type. This is because at runtime, we do not have any type information,
//!         // only unique ids (that change during each compilation).
//!         .resolve::<Arc<dyn Trait2>>("trait2")
//!         .await
//!         .expect("Should exist");
//!     println!("Deep description: {}", trait2.deep_describe());
//! }
//! ```
//!

use async_trait::async_trait;
pub use coi_derive::*;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::sync::Arc;

#[derive(Debug)]
pub enum Error {
    /// This key was not found in the container. Either the requested resource was never registered
    /// with this container, or there is a typo in the register or resolve calls.
    KeyNotFound(String),
    /// The requested key was found in the container, but its type did not match the requested type.
    TypeMismatch(String),
    /// Wrapper around errors produced by `Provider`s.
    Inner(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            Error::KeyNotFound(s) => write!(f, "Key not found: {}", s),
            Error::TypeMismatch(s) => write!(f, "Type mismatch for key: {}", s),
            Error::Inner(ptr) => write!(f, "Inner error: {}", ptr.description()),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::KeyNotFound(_) | Error::TypeMismatch(_) => None,
            Error::Inner(ptr) => Some(ptr.as_ref()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Inject: Send + Sync + 'static {}

impl<T: Inject + ?Sized> Inject for Arc<T> {}

#[derive(Clone)]
pub struct Container {
    provider_map: HashMap<String, Arc<dyn Any + Send + Sync + 'static>>,
}

impl Container {
    /// Construct or lookup a previously constructed object of type `T` with key `key`.
    pub async fn resolve<T>(&self, key: &str) -> Result<T>
    where
        T: Inject,
    {
        let any_provider = self
            .provider_map
            .get(key)
            .ok_or_else(|| Error::KeyNotFound(key.to_owned()))?;
        let provider = any_provider
            .downcast_ref::<Arc<dyn Provide<Output = T> + Send + Sync + 'static>>()
            .ok_or_else(|| Error::TypeMismatch(key.to_owned()))?;
        // FIXME(pfaria) memoize results when singleton registration is introduced
        provider.provide(self).await
    }
}

/// A builder used to construct a `Container`
#[derive(Clone)]
pub struct ContainerBuilder {
    provider_map: HashMap<String, Arc<dyn Any + Send + Sync + 'static>>,
}

impl ContainerBuilder {
    pub fn new() -> Self {
        Self {
            provider_map: HashMap::new(),
        }
    }

    /// Register a `Provider` for `T`.
    pub fn register<K, P, T>(mut self, key: K, provider: P) -> Self
    where
        K: Into<String>,
        T: Inject,
        P: Provide<Output = T> + Send + Sync + 'static,
    {
        let key = key.into();
        let provider = Arc::new(provider) as Arc<dyn Provide<Output = T> + Send + Sync + 'static>;
        self.provider_map.insert(key, Arc::new(provider));
        self
    }

    /// Consumer this builder to produce a `Container`.
    pub fn build(self) -> Container {
        Container {
            provider_map: self.provider_map,
        }
    }
}

#[async_trait]
pub trait Provide {
    /// The type that this provider is intended to produce
    type Output: Inject;

    /// Only intended to be used internally
    async fn provide(&self, container: &Container) -> Result<Self::Output>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ensure_display() {
        use std::io;

        let error = Error::KeyNotFound("S".to_owned());
        let displayed = format!("{}", error);
        assert_eq!(displayed, "Key not found: S");

        let error = Error::TypeMismatch("S2".to_owned());
        let displayed = format!("{}", error);
        assert_eq!(displayed, "Type mismatch for key: S2");

        let error = Error::Inner(Box::new(io::Error::new(io::ErrorKind::NotFound, "oh no!")));
        let displayed = format!("{}", error);
        assert_eq!(displayed, "Inner error: oh no!");
    }

    #[test]
    fn ensure_debug() {
        let error = Error::KeyNotFound("S".to_owned());
        let debugged = format!("{:?}", error);
        assert_eq!(debugged, "KeyNotFound(\"S\")");

        let error = Error::TypeMismatch("S2".to_owned());
        let debugged = format!("{:?}", error);
        assert_eq!(debugged, "TypeMismatch(\"S2\")");
    }

    #[test]
    fn conainer_builder_is_clonable() {
        let builder = ContainerBuilder::new();
        for _ in 0..2 {
            let builder = builder.clone();
            let _container = builder.build();
        }
    }

    #[test]
    fn container_is_clonable() {
        let container = ContainerBuilder::new().build();
        let _container = container.clone();
    }
}
