//! Coi provides an easy to use dependency injection framework.
//! Currently, this crate provides the following:
//! - **[`coi::Provide` (trait)]** - a trait that indicates a struct is capable of providing a
//! specific implementation of some injectable trait. This is generated for you if you use
//! [`coi::provides`] or [`coi::Provide` (derive)], but can also be written manually.
//! - **[`coi::Container`]** - a container to manage the lifetime of all dependencies. This is still
//! in its early stages, and currently only supports objects that are recreated with each request to
//! [`coi::Container::resolve`].
//! - **[`coi::ContainerBuilder`]** - a builder for the above container to simplify construction and
//! guarantee immutability after construction.
//!
//! [`coi::provides`]: attr.provides.html
//! [`coi::Provide` (trait)]: trait.Provide.html
//! [`coi::Provide` (derive)]: derive.Provide.html
//! [`coi::Container`]: struct.Container.html
//! [`coi::Container::resolve`]: struct.Container.html#method.resolve
//! [`coi::ContainerBuilder`]: struct.ContainerBuilder.html
//!
//! # Example
//!
//! ```rust
//! use coi::{coi, container};
//! use std::sync::Arc;
//!
//! // The trait we'd like to provide.
//! trait Trait1 {
//!     fn describe(&self) -> &'static str;
//! }
//!
//! // For structs that will provide the implementation of an injectable trait, specify which expr
//! // will be used to inject which trait. The method can be any path. The arguments for the method
//! // are derived from fields marked with the attribute `#[coi(inject)]` (See Impl2 below).
//! #[coi(provides dyn Trait1 + Send + Sync with Impl1)]
//! struct Impl1;
//!
//! // Don't forget to actually implement the trait.
//! impl Trait1 for Impl1 {
//!     fn describe(&self) -> &'static str {
//!         "I'm impl1!"
//!     }
//! }
//!
//! trait Trait2 {
//!     fn deep_describe(&self) -> String;
//! }
//!
//! // For structs that will provide the implementation of an injectable trait, specify which method
//! // will be used to inject which trait. The arguments for the method are derived from fields
//! // marked with the attribute `#[coi(inject)]`, so the parameter name must match a field name.
//! #[coi(provides dyn Trait2 + Send + Sync with Impl2::new(trait1))]
//! struct Impl2 {
//!     // The name of the field is important! It must match the name that's registered in the
//!     // container when the container is being built! This is similar to the behavior of
//!     // dependency injection libraries in other languages.
//!     #[coi(inject)]
//!     trait1: Arc<dyn Trait1 + Send + Sync>,
//! }
//!
//! // Implement the provider method
//! impl Impl2 {
//!     // Note: The param name here doesn't actually matter.
//!     fn new(trait1: Arc<dyn Trait1 + Send + Sync>) -> Self {
//!         Self { trait1 }
//!     }
//! }
//!
//! // Again, don't forget to actually implement the trait.
//! impl Trait2 for Impl2 {
//!     fn deep_describe(&self) -> String {
//!         format!("I'm impl2! and I have {}", self.trait1.describe())
//!     }
//! }
//!
//! // "Provider" structs are automatically generated through the `#[coi::provides]` attribute.
//! // They append `Provider` to the name of the struct that is being marked (make sure you don't
//! // have any structs with the same name or your code will fail to compile.
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
//!     // Note: Getting the key wrong will produce an error telling you which key in the chain of
//!     // dependencies caused the failure (future versions might provide a vec of the chain that
//!     // lead to the failure). Getting the type wrong will only tell you which key had the wrong
//!     // type. This is because at runtime, we do not have any type information, only unique ids
//!     // (that change during each compilation).
//!     .resolve::<dyn Trait2 + Send + Sync>("trait2")
//!     .expect("Should exist");
//! println!("Deep description: {}", trait2.deep_describe());
//! ```
//!
//! # How this crate works in more detail
//!
//! You'll need a struct that impls `Provide<Output = T>` where `T` is your trait or struct that you
//! wishto have injected. `Provide` exposes a `provide` fn that takes `&self` and `&Container`. When
//! manually implementing `Provide` you must resolve all dependencies with `container`. Here's an
//! example below:
//!
//! ```rust
//! # use coi::{Container, Provide};
//! # use std::sync::{Arc, Mutex};
//! # trait Trait1 {}
//! #
//! trait Dependency {}
//!
//! struct Impl1 {
//!     dependency: Arc<dyn Dependency + Send + Sync>,
//! }
//!
//! impl Impl1 {
//!     fn new(dependency: Arc<dyn Dependency + Send + Sync>) -> Self {
//!         Self { dependency }
//!     }
//! }
//!
//! impl Trait1 for Impl1 {}
//!
//! struct Trait1Provider;
//!
//! impl Provide for Trait1Provider {
//!     type Output = dyn Trait1 + Send + Sync;
//!
//!     fn provide(&self, container: &Container) -> coi::Result<Arc<Self::Output>> {
//!         let dependency = container.resolve::<dyn Dependency + Send + Sync>("dependency")?;
//!         Ok(Arc::new(Impl1::new(dependency)) as Arc<dyn Trait1 + Send + Sync>)
//!     }
//! }
//! ```
//!
//! The `"dependency"` above of course needs to be registered in order for the call
//! to `resolve` to not error out:
//!
//! ```rust
//! # use coi::{container, Container, Provide};
//! # use std::sync::Arc;
//! # trait Trait1 {}
//! # trait Dependency {}
//! #
//! # struct Impl1 {
//! #     dependency: Arc<dyn Dependency + Send + Sync>,
//! # }
//! # impl Impl1 {
//! #     fn new(dependency: Arc<dyn Dependency + Send + Sync>) -> Self {
//! #         Self { dependency }
//! #     }
//! # }
//! # impl Trait1 for Impl1 {}
//! #
//! # struct Trait1Provider;
//! #
//! # impl Provide for Trait1Provider {
//! #     type Output = dyn Trait1 + Send + Sync;
//! #     fn provide(&self, container: &Container) -> coi::Result<Arc<Self::Output>> {
//! #         let dependency = container.resolve::<dyn Dependency + Send + Sync>("dependency")?;
//! #         Ok(Arc::new(Impl1::new(dependency)) as Arc<dyn Trait1 + Send + Sync>)
//! #     }
//! # }
//! struct DepImpl;
//!
//! impl Dependency for DepImpl {}
//!
//! struct DependencyProvider;
//!
//! impl Provide for DependencyProvider {
//!     type Output = dyn Dependency + Send + Sync;
//!
//!     fn provide(&self, _: &Container) -> coi::Result<Arc<Self::Output>> {
//!         Ok(Arc::new(DepImpl) as Arc<dyn Dependency + Send + Sync>)
//!     }
//! }
//!
//! let mut container = container! {
//!     trait1 => Trait1Provider,
//!     dependency => DependencyProvider,
//! };
//! let trait1 = container.resolve::<dyn Trait1 + Send + Sync>("trait1");
//! ```
//!
//! In general, you usually won't want to write all of that. You would instead want to use the
//! procedural macro (see example above).
//! The detailed docs for that are at [`coi::(provide ...)`]
//!
//! # Debugging
//!
//! To turn on debugging features, enable the `debug` feature (see below), then you'll have access
//! to the following changes:
//!
//! *  Formatting a container with `{:?}` will also list the dependencies (in A: Vec&lt;B&gt; style)
//! *  `Container` will get an [`analyze`] fn, which will return an error if any misconfiguration is
//! detected. See the docs for [`analyze`] for more details.
//! *  `Container` will get a [`dot_graph`] fn, which will return a string that can be passed to
//! [graphviz]'s dot command to generate a graph. The image below was generated with the sample
//! project that's in this crate's repository (output saved to `deps.dot` then ran
//! `dot -Tsvg deps.dot -o deps.svg `):
//!
//! <div>
//! <svg width="168pt" height="188pt"
//! viewBox="0.00 0.00 167.89 188.00" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
//! <g id="graph0" class="graph" transform="scale(1 1) rotate(0) translate(4 184)">
//! <title>%3</title>
//! <polygon fill="#ffffff" stroke="transparent" points="-4,4 -4,-184 163.8858,-184 163.8858,4 -4,4"/>
//! <!-- 0 -->
//! <g id="node1" class="node">
//! <title>0</title>
//! <ellipse fill="none" stroke="#000000" cx="79.9429" cy="-18" rx="67.6881" ry="18"/>
//! <text text-anchor="middle" x="79.9429" y="-14.3" font-family="Times,serif" font-size="14.00" fill="#000000">Singleton &#45; pool</text>
//! </g>
//! <!-- 1 -->
//! <g id="node2" class="node">
//! <title>1</title>
//! <ellipse fill="none" stroke="#000000" cx="79.9429" cy="-90" rx="79.8859" ry="18"/>
//! <text text-anchor="middle" x="79.9429" y="-86.3" font-family="Times,serif" font-size="14.00" fill="#000000">Scoped &#45; repository</text>
//! </g>
//! <!-- 1&#45;&gt;0 -->
//! <g id="edge1" class="edge">
//! <title>1&#45;&gt;0</title>
//! <path fill="none" stroke="#000000" d="M79.9429,-71.8314C79.9429,-64.131 79.9429,-54.9743 79.9429,-46.4166"/>
//! <polygon fill="#000000" stroke="#000000" points="83.443,-46.4132 79.9429,-36.4133 76.443,-46.4133 83.443,-46.4132"/>
//! </g>
//! <!-- 2 -->
//! <g id="node3" class="node">
//! <title>2</title>
//! <ellipse fill="none" stroke="#000000" cx="79.9429" cy="-162" rx="69.5877" ry="18"/>
//! <text text-anchor="middle" x="79.9429" y="-158.3" font-family="Times,serif" font-size="14.00" fill="#000000">Scoped &#45; service</text>
//! </g>
//! <!-- 2&#45;&gt;1 -->
//! <g id="edge2" class="edge">
//! <title>2&#45;&gt;1</title>
//! <path fill="none" stroke="#000000" d="M79.9429,-143.8314C79.9429,-136.131 79.9429,-126.9743 79.9429,-118.4166"/>
//! <polygon fill="#000000" stroke="#000000" points="83.443,-118.4132 79.9429,-108.4133 76.443,-118.4133 83.443,-118.4132"/>
//! </g>
//! </g>
//! </svg>
//! </div>
//!
//! [`analyze`]: struct.Container.html#method.analyze
//! [`dot_graph`]: struct.Container.html#method.dot_graph
//! [graphviz]: https://www.graphviz.org/
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
//! Why the #$*%T won't my container work!?
//!
//! To turn on debugging features:
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! coi = { version = "...", default-features = false, features = ["debug"] }
//! ```
//!
//! - default: `derive` - Procedural macros are re-exported.
//! - debug: `Debug` impl
//! - None - Procedural macros are not re-exported.
//!
//! # Help
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

mod container;
#[cfg(feature = "debug")]
#[cfg_attr(docsrs, doc(cfg(feature = "debug")))]
mod debug;
mod macros;
mod provide;
mod registration;
mod resolvable;

pub use crate::container::{Container, ContainerBuilder};
#[cfg(feature = "debug")]
pub use crate::debug::AnalysisError;
#[cfg(feature = "debug")]
pub use crate::provide::Dependencies;
pub use crate::provide::Provide;
pub use crate::registration::RegistrationKind;

#[cfg(any(feature = "derive", feature = "debug"))]
pub use coi_derive::*;

/// Errors produced by this crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This key was not found in the container. Either the requested resource was never registered
    /// with this container, or there is a typo in the register or resolve calls.
    #[error("Key not found: {0}")]
    KeyNotFound(ContainerKey),
    /// The requested key was found in the container, but its type did not match the requested type.
    #[error("Type mismatch for key: {0}")]
    TypeMismatch(ContainerKey),
    /// Wrapper around errors produced by `Provider`s.
    #[error("Inner error: {0}")]
    Inner(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

/// Type alias to `Result<T, coi::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// A simple typedef for `&'static str`
pub type ContainerKey = &'static str;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ensure_display() {
        use std::io;

        let error = Error::KeyNotFound("S");
        let displayed = format!("{}", error);
        assert_eq!(displayed, "Key not found: S");

        let error = Error::TypeMismatch("S2");
        let displayed = format!("{}", error);
        assert_eq!(displayed, "Type mismatch for key: S2");

        let error = Error::Inner(Box::new(io::Error::new(io::ErrorKind::NotFound, "oh no!")));
        let displayed = format!("{}", error);
        assert_eq!(displayed, "Inner error: oh no!");
    }

    #[test]
    fn ensure_debug() {
        let error = Error::KeyNotFound("S");
        let debugged = format!("{:?}", error);
        assert_eq!(debugged, "KeyNotFound(\"S\")");

        let error = Error::TypeMismatch("S2");
        let debugged = format!("{:?}", error);
        assert_eq!(debugged, "TypeMismatch(\"S2\")");
    }
}
