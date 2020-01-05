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
//! # How this crate works
//!
//! For any trait you wish to abstract over, have it inherit the `Inject` trait. For structs, impl
//! `Inject` for that struct, e.g.
//! ```rust
//! # use coi::Inject;
//! trait Trait1: Inject {}
//!
//! struct Struct1;
//!
//! impl Inject for Struct1 {}
//! ```
//!
//! Then, in order to register the injectable item with the [`coi::ContainerBuilder`], you also
//! need a struct that impls `Provide<Output = T>` where `T` is your trait or struct. `Provide`
//! exposes a `provide` fn that takes `&self` and `&Container`. When manually implementing `Provide`
//! you must resolve all dependencies with `container`. Here's an example below:
//!
//! ```rust
//! # #[cfg(any(feature = "async", feature = "derive-async"))] {
//! # use async_trait::async_trait;
//! # use coi::{Container, Inject, Provide};
//! # use std::sync::Arc;
//! # trait Trait1: Inject {}
//! #
//! trait Dependency: Inject {}
//!
//! struct Impl1 {
//!     dependency: Arc<dyn Dependency>,
//! }
//!
//! impl Impl1 {
//!     fn new(dependency: Arc<dyn Dependency>) -> Self {
//!         Self { dependency }
//!     }
//! }
//!
//! impl Inject for Impl1 {}
//!
//! impl Trait1 for Impl1 {}
//!
//! struct Trait1Provider;
//!
//! #[async_trait]
//! impl Provide for Trait1Provider {
//!     type Output = dyn Trait1;
//!
//!     async fn provide(&self, container: &mut Container) -> coi::Result<Arc<Self::Output>> {
//!         let dependency = container.resolve::<dyn Dependency>("dependency").await?;
//!         Ok(Arc::new(Impl1::new(dependency)) as Arc<dyn Trait1>)
//!     }
//! }
//! # }
//! ```
//!
//! The `"dependency"` above of course needs to be registered in order for the call
//! to `resolve` to not error out:
//!
//! ```rust
//! # #[cfg(any(feature = "async", feature="derive-async"))] {
//! # use async_trait::async_trait;
//! # use coi::{Container, ContainerBuilder, Inject, Provide};
//! # use std::sync::Arc;
//! # trait Trait1: Inject {}
//! # trait Dependency: Inject {}
//! #
//! # struct Impl1 {
//! #     dependency: Arc<dyn Dependency>,
//! # }
//! # impl Impl1 {
//! #     fn new(dependency: Arc<dyn Dependency>) -> Self {
//! #         Self { dependency }
//! #     }
//! # }
//! # impl Inject for Impl1 {}
//! # impl Trait1 for Impl1 {}
//! #
//! # struct Trait1Provider;
//! #
//! # #[async_trait]
//! # impl Provide for Trait1Provider {
//! #     type Output = dyn Trait1;
//! #     async fn provide(&self, container: &mut Container) -> coi::Result<Arc<Self::Output>> {
//! #         let dependency = container.resolve::<dyn Dependency>("dependency").await?;
//! #         Ok(Arc::new(Impl1::new(dependency)) as Arc<dyn Trait1>)
//! #     }
//! # }
//! struct DepImpl;
//!
//! impl Dependency for DepImpl {}
//!
//! impl Inject for DepImpl {}
//!
//! struct DependencyProvider;
//!
//! #[async_trait]
//! impl Provide for DependencyProvider {
//!     type Output = dyn Dependency;
//!
//!     async fn provide(&self, _: &mut Container) -> coi::Result<Arc<Self::Output>> {
//!         Ok(Arc::new(DepImpl) as Arc<dyn Dependency>)
//!     }
//! }
//!
//! async move {
//!     let mut container = container! {
//!         trait1 => Trait1Provider,
//!         dependency => DependencyProvider,
//!     };
//!     let trait1 = container.resolve::<dyn Trait1>("trait1").await;
//! };
//! # }
//! ```
//!
//! In general, you usually won't want to write all of that. You would instead want to use the
//! procedural macro (see example at the bottom).
//! The detailed docs for that are at [`coi::Inject` (derive)]
//!
//! # Example
//!
//! ```rust
//! # #[cfg(feature = "derive-async")] {
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
//! async move {
//!     // "Provider" structs are automatically generated through the `Inject` attribute. They
//!     // append `Provider` to the name of the struct that is being derive (make sure you don't
//!     // any structs with the same name or your code will fail to compile.
//!     // Reminder: Make sure you use the same key here as the field names of the structs that
//!     // require these impls.
//!     let mut container = container! {
//!         trait1 => Impl1Provider,
//!         trait2 => Impl2Provider,
//!     };
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
//!         .resolve::<dyn Trait2>("trait2")
//!         .await
//!         .expect("Should exist");
//!     println!("Deep description: {}", trait2.deep_describe());
//! };
//! # }
//! ```

use std::any::Any;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::sync::Arc;

#[cfg(any(feature = "derive", feature = "derive-async"))]
pub use coi_derive::*;

#[cfg(feature = "async")]
pub use async_trait::async_trait;
#[cfg(feature = "async")]
use futures::future::{BoxFuture, FutureExt};

#[cfg(feature = "async")]
type Mutex<T> = async_std::sync::Mutex<T>;
#[cfg(not(feature = "async"))]
type Mutex<T> = std::sync::Mutex<T>;

/// Errors produced by this crate
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

/// Type alias to `Result<T, coi::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// A marker trait for injectable traits and structs.
pub trait Inject: Send + Sync + 'static {}

impl<T: Inject + ?Sized> Inject for Arc<T> {}

/// Control how the container will call a provider
#[derive(Clone)]
pub enum Registration<T> {
    /// The container will construct a new instance of `T` for every invocation
    /// of `T::Provide`.
    Normal(T),
    /// The container will construct a new instance of `T` for each scope
    /// container created through `Container::scoped`.
    Scoped(T),
    /// The container will construct a single instance of `T` and reuse it
    /// throughout all scopes.
    Singleton(T),
}

impl<T> Registration<T> {
    fn as_ref(&self) -> Registration<&T> {
        match self {
            Registration::Normal(t) => Registration::Normal(t),
            Registration::Scoped(t) => Registration::Scoped(t),
            Registration::Singleton(t) => Registration::Singleton(t),
        }
    }

    fn map<F, U>(self, func: F) -> Registration<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            Registration::Normal(t) => Registration::Normal(func(t)),
            Registration::Scoped(t) => Registration::Scoped(func(t)),
            Registration::Singleton(t) => Registration::Singleton(func(t)),
        }
    }
}

/// A struct that manages all injected types.
#[derive(Clone)]
pub struct Container {
    provider_map: HashMap<String, Registration<Arc<dyn Any + Send + Sync>>>,
    resolved_map: HashMap<String, Arc<dyn Any + Send + Sync>>,
    parent: Option<Arc<Mutex<Container>>>,
}

macro_rules! lock {
    ($mutex:expr => await) => {
        $mutex.lock().await
    };
    ($mutex:expr) => {
        $mutex.lock().unwrap()
    };
}

macro_rules! resolve {
    (@result_ty $t:ty) => {
        Result<Arc<$t>>
    };
    (@await $expr:expr, await) => {
        $expr.await
    };
    (@await $expr:expr) => {
        $expr
    };
    (@inner <$T:ty> $self:ident $key:ident $($await:ident)?) => {
        // If we already have a resolved version, return it.
        if $self.resolved_map.contains_key($key) {
            return $self
                .resolved_map
                .get($key)
                .unwrap()
                .downcast_ref::<Arc<$T>>()
                .map(Arc::clone)
                .ok_or_else(|| Error::TypeMismatch($key.to_owned()));
        }

        // Try to find the provider
        let any_provider = match $self.provider_map.get($key) {
            Some(provider) => provider,
            None => {
                // If the key is not found, then we might be a child container. If we have a
                // parent, then search it for a possibly valid provider.
                return match &$self.parent {
                    Some(parent) => resolve!(
                        @await
                        lock!(parent $(=> $await)?).resolve::<$T>($key) $(, $await)?
                    ),
                    None => Err(Error::KeyNotFound($key.to_owned())),
                };
            }
        };

        let provider = any_provider.as_ref().map(|p| {
            p.downcast_ref::<Arc<dyn Provide<Output = $T> + Send + Sync + 'static>>()
                .map(Arc::clone)
                .ok_or_else(|| Error::TypeMismatch($key.to_owned()))
        });

        match provider {
            Registration::Normal(p) => Ok(resolve!(@await p?.provide($self) $(, $await)?)?),
            Registration::Scoped(p) | Registration::Singleton(p) => {
                let provided = resolve!(@await p?.provide($self) $(, $await)?)?;
                $self.resolved_map.insert($key.to_owned(), Arc::new(provided));
                Ok($self.resolved_map[$key]
                    .downcast_ref::<Arc<$T>>()
                    .map(Arc::clone)
                    .unwrap())
            }
        }
    };
    (@async_wrapped $self:ident $key:ident) => {
        async move {
            resolve!(@inner $self $key await T)
        }
    };
    (@def async) => {
        pub fn resolve<'a, 'b, 'c, T>(
            &'a mut self,
            key: &'b str
        ) -> BoxFuture<'c, resolve!(@result_ty T)>
        where
            'a: 'c,
            'b: 'c,
            T: Inject + ?Sized,
        {
            async move {
                resolve!{@inner <T> self key await}
            }
            .boxed()
        }
    };
    (@def) => {
        pub fn resolve<T>(&mut self, key: &str) -> resolve!(@result_ty T)
        where
            T: Inject + ?Sized,
        {
            resolve!{ @inner <T> self key }
        }
    };
    ($($async:ident)?) => {
        /// Construct or lookup a previously constructed object of type `T` with key `key`.
        resolve!{@def $($async)? }
    }
}

impl Container {
    #[cfg(feature = "async")]
    resolve! {async}

    #[cfg(not(feature = "async"))]
    resolve! {}

    pub fn scopable(self) -> Scopable {
        Scopable(Arc::new(Mutex::new(self)))
    }
}

pub struct Scopable(Arc<Mutex<Container>>);

macro_rules! scoped {
    (@fn $($async:ident $await:ident)?) => {
        /// Produce a child container that only contains providers for scoped registrations
        /// Any calls to resolve from the returned container can still use the `self` container
        /// to resolve any other kinds of registrations.
        pub $($async)? fn scoped(&self) -> Container {
            Container {
                provider_map: lock!(self.0 $(=> $await)?)
                    .provider_map
                    .iter()
                    .filter_map(|(k, v)| match v {
                        Registration::Scoped(v) => {
                            Some((k.clone(), Registration::Scoped(Arc::clone(v))))
                        },
                        _ => None,
                    })
                    .collect(),
                resolved_map: HashMap::new(),
                parent: Some(Arc::clone(&self.0)),
            }
        }
    };
    (async) => {
        scoped!(@fn async await);
    };
    () => {
        scoped!(@fn);
    }
}

impl Scopable {
    #[cfg(feature = "async")]
    scoped!(async);

    #[cfg(not(feature = "async"))]
    scoped!();
}

/// A builder used to construct a `Container`.
#[derive(Clone, Default)]
pub struct ContainerBuilder {
    provider_map: HashMap<String, Registration<Arc<dyn Any + Send + Sync>>>,
}

impl ContainerBuilder {
    pub fn new() -> Self {
        Self {
            provider_map: HashMap::new(),
        }
    }

    /// Register a `Provider` for `T` with identifier `key`.
    #[inline]
    pub fn register<K, P, T>(self, key: K, provider: P) -> Self
    where
        K: Into<String>,
        T: Inject + ?Sized,
        P: Provide<Output = T> + Send + Sync + 'static,
    {
        self.register_as(key, Registration::Normal(provider))
    }

    fn get_arc<P, T>(provider: P) -> Arc<dyn Provide<Output = T> + Send + Sync>
    where
        T: Inject + ?Sized,
        P: Provide<Output = T> + Send + Sync + 'static,
    {
        Arc::new(provider)
    }

    /// Register a `Provider` for `T` with identifier `key`, while also specifying the resolution
    /// behavior.
    pub fn register_as<K, P, T>(mut self, key: K, provider: Registration<P>) -> Self
    where
        K: Into<String>,
        T: Inject + ?Sized,
        P: Provide<Output = T> + Send + Sync + 'static,
    {
        let key = key.into();
        self.provider_map.insert(
            key,
            provider.map(|p| Arc::new(Self::get_arc(p)) as Arc<dyn Any + Send + Sync>),
        );
        self
    }

    /// Consumer this builder to produce a `Container`.
    pub fn build(self) -> Container {
        Container {
            provider_map: self.provider_map,
            resolved_map: HashMap::new(),
            parent: None,
        }
    }
}

#[cfg(feature = "async")]
#[async_trait]
pub trait Provide {
    type Output: Inject + ?Sized;

    async fn provide(&self, container: &mut Container) -> Result<Arc<Self::Output>>;
}

#[cfg(not(feature = "async"))]
pub trait Provide {
    type Output: Inject + ?Sized;

    fn provide(&self, container: &mut Container) -> Result<Arc<Self::Output>>;
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

#[macro_export]
macro_rules! container {
    (@registration $provider:ident scoped) => {
        ::coi::Registration::Scoped($provider)
    };
    (@registration $provider:ident singleton) => {
        ::coi::Registration::Singleton($provider)
    };
    (@registration $provider:ident normal) => {
        ::coi::Registration::Normal($provider)
    };
    (@registration $provider:ident) => {
        ::coi::Registration::Normal($provider)
    };
    (@line $builder:ident $key:ident $provider:ident $($call:ident)?) => {
        $builder = $builder.register_as(stringify!($key), container!(@registration $provider $($call)?));
    };
    ($($key:ident => $provider:ident $(. $call:ident)?),+) => {
        container!{ $( $key => $provider $(. $call)?, )+ }
    };
    ($($key:ident => $provider:ident $(. $call:ident)?,)+) => {
        {
            let mut builder = ::coi::ContainerBuilder::new();
            $(container!(@line builder $key $provider $($call)?);)+
            builder.build()
        }
    }
}
