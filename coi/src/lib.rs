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
//! # use coi::{Container, Inject, Provide};
//! # use std::sync::{Arc, Mutex};
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
//! impl Provide for Trait1Provider {
//!     type Output = dyn Trait1;
//!
//!     fn provide(&self, container: &Container) -> coi::Result<Arc<Self::Output>> {
//!         let dependency = container.resolve::<dyn Dependency>("dependency")?;
//!         Ok(Arc::new(Impl1::new(dependency)) as Arc<dyn Trait1>)
//!     }
//! }
//! ```
//!
//! The `"dependency"` above of course needs to be registered in order for the call
//! to `resolve` to not error out:
//!
//! ```rust
//! # use coi::{container, Container, Inject, Provide};
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
//! # impl Provide for Trait1Provider {
//! #     type Output = dyn Trait1;
//! #     fn provide(&self, container: &Container) -> coi::Result<Arc<Self::Output>> {
//! #         let dependency = container.resolve::<dyn Dependency>("dependency")?;
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
//! impl Provide for DependencyProvider {
//!     type Output = dyn Dependency;
//!
//!     fn provide(&self, _: &Container) -> coi::Result<Arc<Self::Output>> {
//!         Ok(Arc::new(DepImpl) as Arc<dyn Dependency>)
//!     }
//! }
//!
//! let mut container = container! {
//!     trait1 => Trait1Provider,
//!     dependency => DependencyProvider,
//! };
//! let trait1 = container.resolve::<dyn Trait1>("trait1");
//! ```
//!
//! In general, you usually won't want to write all of that. You would instead want to use the
//! procedural macro (see example below).
//! The detailed docs for that are at [`coi::Inject` (derive)]
//!
//! # Example
//!
//! ```rust
//! use coi::{container, Inject};
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
//! // "Provider" structs are automatically generated through the `Inject` attribute. They
//! // append `Provider` to the name of the struct that is being derive (make sure you don't
//! // any structs with the same name or your code will fail to compile.
//! // Reminder: Make sure you use the same key here as the field names of the structs that
//! // require these impls.
//! let mut container = container! {
//!     trait1 => Impl1Provider,
//!     trait2 => Impl2Provider,
//! };
//!
//! // Once the container is built, you can now resolve any particular instance by its key and
//! // the trait it provides. This crate currently only supports `Arc<dyn Trait>`, but this may
//! // be expanded in a future version of the crate.
//! let trait2 = container
//!     // Note: Getting the key wrong will produce an error telling you which key in the
//!     // chain of dependencies caused the failure (future versions might provider a vec of
//!     // chain that lead to the failure). Getting the type wrong will only tell you which key
//!     // had the wrong type. This is because at runtime, we do not have any type information,
//!     // only unique ids (that change during each compilation).
//!     .resolve::<dyn Trait2>("trait2")
//!     .expect("Should exist");
//! println!("Deep description: {}", trait2.deep_describe());
//! ```
//!
//! # Features
//!
//! Compilation taking too long? Turn off features you're not using.
//!
//! To not use the default:
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! coi = { version = "...", default-features = false }
//! ```
//!
//! - default: `derive` - Procedural macros are re-exported.
//! - None - Procedural macros are not re-exported.
//!
//! # Help
//!
//! ## External traits
//!
//! Want to inject a trait that's not marked `Inject`? There's a very simple solution!
//! It works even if the intended trait is not part of your crate.
//! ```rust
//! # use coi::Inject;
//! // other.rs
//! pub trait Trait {
//! # /*
//!     ...
//! # */
//! }
//!
//! // your_lib.rs
//! # /*
//! use coi::Inject;
//! use other::Trait;
//! # */
//!
//! // Just inherit the intended trait and `Inject` on a trait in your crate,
//! // and make sure to also impl both traits for the intended provider.
//! pub trait InjectableTrait : Trait + Inject {}
//!
//! #[derive(Inject)]
//! #[provides(pub dyn InjectableTrait with Impl{})]
//! struct Impl {
//! # /*
//!     ...
//! # */
//! }
//!
//! impl Trait for Impl {
//! # /*
//!     ...
//! # */
//! }
//!
//! impl InjectableTrait for Impl {}
//! ```
//!
//! ## Where are the factory registrations!?
//!
//! If you're familiar with dependency injection in other languages, you might
//! be used to factory registration where you can provide a method/closure/lambda/etc.
//! during registration. Since the crate works off of the `Provide` trait, you would
//! have to manually implement `Provide` for your factory method. This would also
//! require you to manually retrieve your dependencies from the passed in `Container`
//! as shown in the docs above.
//!
//! ## Why can't I derive `Inject` when my struct contains a reference?
//!
//! In order to store all of the resolved types, we have to use
//! [`std::any::Any`], which, unfortunately, has the restriction `Any: 'static`.
//! This is because it's not yet known if there's a safe way to downcast to a
//! type with a reference (See the comments in this [tracking issue]).
//!
//! [`std::any::Any`]: https://doc.rust-lang.org/std/any/trait.Any.html
//! [tracking issue]: https://github.com/rust-lang/rust/issues/41875

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(any(feature = "derive", feature = "debug"))]
pub use coi_derive::*;
#[cfg(feature = "debug")]
use std::fmt::Debug;

/// Errors produced by this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This key was not found in the container. Either the requested resource was never registered
    /// with this container, or there is a typo in the register or resolve calls.
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    /// The requested key was found in the container, but its type did not match the requested type.
    #[error("Type mismatch for key: {0}")]
    TypeMismatch(String),
    /// Wrapper around errors produced by `Provider`s.
    #[error("Inner error: {0}")]
    Inner(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

/// Type alias to `Result<T, coi::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// A marker trait for injectable traits and structs.
pub trait Inject: Send + Sync + 'static {}

impl<T: Inject + ?Sized> Inject for Arc<T> {}

/// Control when `Container` will call `Provide::provide`.
#[derive(Copy, Clone, Debug)]
pub enum RegistrationKind {
    /// `Container` will construct a new instance of `T` for every invocation
    /// of `Container::resolve`.
    ///
    /// # Example
    /// ```rust
    /// # use coi::{container, Inject, Result};
    /// # use std::ops::Deref;
    /// # trait Trait: Inject {}
    /// # #[derive(Inject)]
    /// # #[provides(dyn Trait with Impl)]
    /// # struct Impl;
    /// # impl Trait for Impl {}
    /// # fn the_test() -> Result<()> {
    /// let mut container = container! {
    ///     // same as trait => ImplProvider.transient
    ///     trait => ImplProvider
    /// };
    ///
    /// let instance_1 = container.resolve::<dyn Trait>("trait")?;
    /// let instance_2 = container.resolve::<dyn Trait>("trait")?;
    ///
    /// // Every instance resolved from the container will be a distinct instance.
    /// assert_ne!(
    ///     instance_1.deref() as &dyn Trait as *const _,
    ///     instance_2.deref() as &dyn Trait as *const _
    /// );
    /// # Ok(())
    /// # }
    /// # the_test().unwrap()
    /// ```
    Transient,
    /// `Container` will construct a new instance of `T` for each scope
    /// container created through `Container::scoped`.
    ///
    /// # Example
    /// ```rust
    /// # use coi::{container, Inject, Result};
    /// # use std::{ops::Deref, sync::{Arc, Mutex}};
    /// # trait Trait: Inject {}
    /// # #[derive(Inject)]
    /// # #[provides(dyn Trait with Impl)]
    /// # struct Impl;
    /// # impl Trait for Impl {}
    /// # fn the_test() -> Result<()> {
    /// let container = container! {
    ///     trait => ImplProvider.scoped
    /// };
    ///
    /// // Every instance resolved within the same scope will be the same instance.
    /// let instance_1 = container.resolve::<dyn Trait>("trait")?;
    /// let instance_2 = container.resolve::<dyn Trait>("trait")?;
    /// assert_eq!(
    ///     instance_1.deref() as &dyn Trait as *const _,
    ///     instance_2.deref() as &dyn Trait as *const _
    /// );
    /// {
    ///     let scoped = container.scoped();
    ///     let instance_3 = scoped.resolve::<dyn Trait>("trait")?;
    ///
    ///     // Since these two were resolved in different scopes, they will never be the
    ///     // same instance.
    ///     assert_ne!(
    ///         instance_1.deref() as &dyn Trait as *const _,
    ///         instance_3.deref() as &dyn Trait as *const _
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// # the_test().unwrap()
    /// ```
    Scoped,
    /// The container will construct a single instance of `T` and reuse it
    /// throughout all scopes.
    ///
    /// # Example
    /// ```rust
    /// # use coi::{container, Inject, Result};
    /// # use std::{ops::Deref, sync::{Arc, Mutex}};
    /// # trait Trait: Inject {}
    /// # #[derive(Inject)]
    /// # #[provides(dyn Trait with Impl)]
    /// # struct Impl;
    /// # impl Trait for Impl {}
    /// # fn the_test() -> Result<()> {
    /// let container = container! {
    ///     trait => ImplProvider.singleton
    /// };
    ///
    /// let instance_1 = container.resolve::<dyn Trait>("trait")?;
    /// let instance_2 = container.resolve::<dyn Trait>("trait")?;
    ///
    /// assert_eq!(
    ///     instance_1.deref() as &dyn Trait as *const _,
    ///     instance_2.deref() as &dyn Trait as *const _
    /// );
    /// {
    ///     let scoped = container.scoped();
    ///     let instance_3 = scoped.resolve::<dyn Trait>("trait")?;
    ///
    ///     // Regardless of what scope the instance was resolved it, it will always
    ///     // be the same instance.
    ///     assert_eq!(
    ///         instance_1.deref() as &dyn Trait as *const _,
    ///         instance_3.deref() as &dyn Trait as *const _
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// # the_test().unwrap()
    /// ```
    Singleton,
}

#[derive(Clone, Debug)]
pub struct Registration<T> {
    kind: RegistrationKind,
    provider: T,
}

impl<T> Registration<T> {
    pub fn new(kind: RegistrationKind, provider: T) -> Self {
        Self { kind, provider }
    }
}

/// A struct that manages all injected types.
#[derive(Clone, Debug)]
struct InnerContainer {
    provider_map: HashMap<String, Registration<Arc<dyn Any + Send + Sync>>>,
    resolved_map: HashMap<String, Arc<dyn Any + Send + Sync>>,
    parent: Option<Container>,
    #[cfg(feature = "debug")]
    dependency_map: HashMap<String, Vec<&'static str>>,
}

impl InnerContainer {
    fn check_resolved<T>(&self, key: &str) -> Option<Result<Arc<T>>>
    where
        T: Inject + ?Sized,
    {
        self.resolved_map.get(key).map(|v| {
            v.downcast_ref::<Arc<T>>()
                .map(Arc::clone)
                .ok_or_else(|| Error::TypeMismatch(key.to_owned()))
        })
    }
}

#[derive(Clone, Debug)]
pub struct Container(Arc<Mutex<InnerContainer>>);

impl Container {
    fn new(container: InnerContainer) -> Self {
        Self(Arc::new(Mutex::new(container)))
    }

    pub fn resolve<T>(&self, key: &str) -> Result<Arc<T>>
    where
        T: Inject + ?Sized,
    {
        let (kind, provider) = {
            let container = self.0.lock().unwrap();
            // If we already have a resolved version, return it.
            if let Some(resolved) = container.check_resolved::<T>(key) {
                return resolved;
            }

            // Try to find the provider
            let registration = match container.provider_map.get(key) {
                Some(provider) => provider,
                None => {
                    // If the key is not found, then we might be a child container. If we have a
                    // parent, then search it for a possibly valid provider.
                    return match &container.parent {
                        Some(parent) => {
                            let parent = parent.clone();
                            // Release the lock so we don't deadlock, this container isn't
                            // needed anymore
                            parent.resolve::<T>(key)
                        }
                        None => Err(Error::KeyNotFound(key.to_owned())),
                    };
                }
            };

            (
                registration.kind,
                registration
                    .provider
                    .downcast_ref::<Arc<dyn Provide<Output = T> + Send + Sync + 'static>>()
                    .map(Arc::clone)
                    .ok_or_else(|| Error::TypeMismatch(key.to_owned()))?,
            )
        };
        let provided = provider.provide(self);

        match kind {
            RegistrationKind::Transient => provided,
            RegistrationKind::Scoped | RegistrationKind::Singleton => {
                let mut container = self.0.lock().unwrap();
                // Since there's a possibility for a deadlock right now, we want to make sure
                // no one else already inserted into the resolved map (hence the call to entry).
                Ok(container
                    .resolved_map
                    .entry(key.to_owned())
                    .or_insert(Arc::new(provided?))
                    .downcast_ref::<Arc<T>>()
                    .map(Arc::clone)
                    .unwrap())
            }
        }
    }

    /// Produce a child container that only contains providers for scoped registrations
    /// Any calls to resolve from the returned container can still use the `self` container
    /// to resolve any other kinds of registrations.
    pub fn scoped(&self) -> Container {
        let container: &InnerContainer = &self.0.lock().unwrap();
        Container::new(InnerContainer {
            provider_map: container
                .provider_map
                .iter()
                .filter_map(|(k, v)| match v.kind {
                    kind @ RegistrationKind::Scoped | kind @ RegistrationKind::Transient => Some((
                        k.clone(),
                        Registration {
                            kind,
                            provider: Arc::clone(&v.provider),
                        },
                    )),
                    _ => None,
                })
                .collect(),
            resolved_map: HashMap::new(),
            // FIXME(pfaria) no clone here
            #[cfg(feature = "debug")]
            dependency_map: container.dependency_map.clone(),
            parent: Some(self.clone()),
        })
    }
}

/// A builder used to construct a `Container`.
#[derive(Clone, Default)]
pub struct ContainerBuilder {
    provider_map: HashMap<String, Registration<Arc<dyn Any + Send + Sync>>>,
    #[cfg(feature = "debug")]
    dependency_map: HashMap<String, Vec<&'static str>>,
}

impl ContainerBuilder {
    /// Constructor for `ContainerBuilder`.
    pub fn new() -> Self {
        Self {
            provider_map: HashMap::new(),
            #[cfg(feature = "debug")]
            dependency_map: HashMap::new(),
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
        self.register_as(
            key,
            Registration::new(RegistrationKind::Transient, provider),
        )
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
    pub fn register_as<K, P, T>(mut self, key: K, registration: Registration<P>) -> Self
    where
        K: Into<String>,
        T: Inject + ?Sized,
        P: Provide<Output = T> + Send + Sync + 'static,
    {
        let key = key.into();
        #[cfg(feature = "debug")]
        let deps = registration.provider.dependencies();
        self.provider_map.insert(
            #[cfg(feature = "debug")]
            {
                key.clone()
            },
            #[cfg(not(feature = "debug"))]
            {
                key
            },
            Registration {
                kind: registration.kind,
                provider: Arc::new(Self::get_arc(registration.provider))
                    as Arc<dyn Any + Send + Sync>,
            },
        );
        #[cfg(feature = "debug")]
        self.dependency_map.insert(key, deps);
        self
    }

    /// Consume this builder to produce a `Container`.
    pub fn build(self) -> Container {
        Container::new(InnerContainer {
            provider_map: self.provider_map,
            resolved_map: HashMap::new(),
            parent: None,
            #[cfg(feature = "debug")]
            dependency_map: self.dependency_map,
        })
    }
}

/// A trait to manage the construction of an injectable trait or struct.
pub trait Provide {
    /// The type that this provider is intended to produce
    type Output: Inject + ?Sized;

    /// Only intended to be used internally
    fn provide(&self, container: &Container) -> Result<Arc<Self::Output>>;

    /// Return list of dependencies
    #[cfg(feature = "debug")]
    fn dependencies(&self) -> Vec<&'static str>;
}

/// A macro to simplify building of `Container`s.
///
/// It takes a list of key-value pairs, where the keys are converted to string
/// keys, and the values are converted into registrations. Transient, singleton
/// and scoped registrations are possible, with transient being the default:
/// ```rust
/// use coi::{container, Inject};
///
/// trait Dep: Inject {}
///
/// #[derive(Inject)]
/// #[provides(dyn Dep with Impl)]
/// struct Impl;
///
/// impl Dep for Impl {}
///
/// let mut container = container! {
///     dep => ImplProvider,
///     transient_dep => ImplProvider.transient,
///     singleton_dep => ImplProvider.singleton,
///     scoped_dep => ImplProvider.scoped
/// };
/// ```
///
/// For details on how each registration works, see [`coi::Registration`]
///
/// [`coi::Registration`]: enum.Registration.html
#[macro_export]
macro_rules! container {
    (@registration $provider:ident scoped) => {
        $crate::Registration::new(
            $crate::RegistrationKind::Scoped,
            $provider
        )
    };
    (@registration $provider:ident singleton) => {
        $crate::Registration::new(
            $crate::RegistrationKind::Singleton,
            $provider
        )
    };
    (@registration $provider:ident transient) => {
        $crate::Registration::new(
            $crate::RegistrationKind::Transient,
            $provider
        )
    };
    (@registration $provider:ident) => {
        $crate::Registration::new(
            $crate::RegistrationKind::Transient,
            $provider
        )
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
