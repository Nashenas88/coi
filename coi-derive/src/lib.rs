#![deny(missing_docs)]
//! Coi-derive simplifies implementing the traits provided in the [coi] crate.
//!
//! [coi]: https://docs.rs/coi

extern crate proc_macro;

mod attr;
mod ctxt;
mod provides;
mod symbol;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

use crate::attr::Provides;

/// Generates a "Provider" struct with its own `Provide` impl.
///
/// It takes the form
/// ```rust,ignore
/// #[coi(provides <vis> <ty> with <expr>)]
/// ```
/// or the form
/// ```rust,ignore
/// #[coi(provides <vis> <ty> as <name> with <expr>)]
/// ```
/// The latter form *must* be used when generating multiple providers for a single type. This might
/// be useful if you have multiple trait implementations for one struct and want to provide separate
/// unique instances for each trait in the container. That use case might be more common with mocks
/// in unit tests rather than in production code.
///
/// It generates a provider struct with visibility `<vis>` that impls `Provide` with an output type
/// of `Arc<<ty>>`. It will construct `<ty>` with `<expr>`, and all params to `<expr>` must match
/// the struct fields marked with `#[coi::(inject)]` (see the next bullet item). `<vis>` must match
/// the visibility of `<ty>` or you will get code that might not compile. If `<name>` is not
/// provided, the struct name will be used and `Provider` will be appended to it.
/// - `#[coi(inject)]` - All fields marked `#[coi(inject)]` are resolved in the `provide` fn
/// described above.
/// Given a field `<field_name>: Arc<<field_ty>>`, this attribute will cause the following resolution to
/// be generated:
/// ```rust,ignore
/// let <field_name> = Container::resolve::<<field_ty>>(container, "<field_name>");
/// ```
/// Because of this, it's important that the field name *must* match the string that's used to
/// register the provider in the `ContainerBuilder`.
///
/// ## Examples
///
/// Private trait and no dependencies
/// ```rust
/// # /*
/// use coi::coi
/// # */
/// # use coi_derive::coi;
///
/// trait Priv {}
///
/// #[coi(provides dyn Priv + Send + Sync with SimpleStruct)]
/// # pub
/// struct SimpleStruct;
///
/// impl Priv for SimpleStruct {}
/// ```
///
/// Public trait and dependency
/// ```rust
/// # /*
/// use coi::coi
/// # */
/// # use coi_derive::coi;
/// use std::sync::Arc;
///
/// pub trait Pub {}
/// pub trait Dependency {}
///
/// #[coi(provides pub dyn Pub + Send + Sync with NewStruct::new(dependency))]
/// # pub
/// struct NewStruct {
///     #[coi(inject)]
///     dependency: Arc<dyn Dependency + Send + Sync>,
/// }
///
/// impl NewStruct {
///     fn new(dependency: Arc<dyn Dependency + Send + Sync>) -> Self {
///         Self {
///             dependency
///         }
///     }
/// }
///
/// impl Pub for NewStruct {}
/// ```
///
/// Struct injection
/// ```rust
/// # /*
/// use coi::coi
/// # */
/// # use coi_derive::coi;
///
/// #[coi(provides pub InjectableStruct with InjectableStruct)]
/// # pub
/// struct InjectableStruct;
/// ```
///
/// Unnamed fields
/// ```rust
/// # /*
/// use coi::coi
/// # */
/// # use coi_derive::coi;
/// use std::sync::Arc;
///
/// #[coi(provides Dep1 with Dep1)]
/// struct Dep1;
///
/// #[coi(provides Impl1 with Impl1(dep1))]
/// struct Impl1(#[coi(inject = "dep1")] Arc<Dep1>);
/// ```
///
/// Generics
/// ```rust
/// use coi::{
///    container,
/// # /*
///    coi,
/// # */
/// };
/// # use coi_derive::coi;
///
/// #[coi(provides Impl1<T> with Impl1::<T>::new())]
/// struct Impl1<T>(T)
/// where
///     T: Default;
///
/// impl<T> Impl1<T>
/// where
///     T: Default,
/// {
///     fn new() -> Self {
///         Self(Default::default())
///     }
/// }
///
/// fn build_container() {
///   // Take note that these providers have to be constructed
///   // with explicit types.
///   let impl1_provider = Impl1Provider::<bool>::new();
///   let container = container! {
///       impl1 => impl1_provider,
///   };
///   let _bool_impl = container
///       .resolve::<Impl1<bool>>("impl1")
///       .expect("Should exist");
/// }
///
/// # build_container();
/// ```
///
/// If you need some form of constructor fn that takes arguments that are not injected, then you
/// might be able to use the [`coi::Provide`] derive. If that doesn't fit your use case, you'll
/// need to manually implement `Provide`.
///
/// [`coi::Provide`]: derive.Provide.html
#[proc_macro_attribute]
pub fn coi(attr: TokenStream, item: TokenStream) -> TokenStream {
    let provides = parse_macro_input!(attr as Provides);
    let input = parse_macro_input!(item as ItemStruct);
    provides::provides_attr(provides, input, cfg!(feature = "debug")).into()
}

/// Generates an impl for `Provide` and also generates a "Provider" struct with its own
/// `Provide` impl.
///
/// This derive proc macro impls `Provide` on the struct it modifies, and also processes #[coi(provides ...)]
/// attributes:
/// - `#[coi(provides ...)]` - It takes the form
/// ```rust,ignore
/// use coi::Provide
///
/// #[derive(Provide)]
/// #[coi(provides <vis> <ty> with <expr>)]
/// ```
///
/// Multiple `provides` attributes are not allowed since this is for a specific `Provide` impl and
/// not for the resolved type.
///
/// It generates a provider struct with visibility `<vis>`
/// that impls `Provide` with an output type of `Arc<<ty>>`. It will construct `<ty>` with `<expr>`,
/// and all params to `<expr>` must match the struct fields marked with `#[coi(inject)]` (see the
/// next bullet item). `<vis>` must match the visibility of `<ty>` or you will get code that might
/// not compile. If `<name>` is not provided, the struct name will be used and `Provider` will be
/// appended to it.
///
/// ## Examples
///
/// Private trait and no dependencies
/// ```rust
/// # /*
/// use coi::Provide;
/// # */
/// # use coi::Provide;
/// # use coi_derive::Provide;
///
/// trait Priv {}
///
/// # pub
/// struct SimpleStruct {
///     data: u32
/// }
///
/// impl SimpleStruct {
///     fn new(data: u32) -> Self {
///         Self { data }
///     }
/// }
///
/// impl Priv for SimpleStruct {}
///
/// #[derive(Provide)]
/// #[coi(provides dyn Priv + Send + Sync with SimpleStruct::new(self.data))]
/// struct SimpleStructProvider {
///     data: u32,
/// }
///
/// impl SimpleStructProvider {
///     fn new(data: u32) -> Self {
///         Self { data }
///     }
/// }
/// ```
#[proc_macro_derive(Provide, attributes(coi))]
pub fn provide_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    provides::provides_derive(input, cfg!(feature = "debug")).into()
}
