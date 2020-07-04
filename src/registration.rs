/// Control when `Container` will call `Provide::provide`.
#[derive(Copy, Clone, Debug)]
pub enum RegistrationKind {
    /// The container will construct a single instance of `T` and reuse it
    /// throughout all scopes.
    ///
    /// # Example
    /// ```rust
    /// # use coi::{coi, container, Result};
    /// # use std::{ops::Deref, sync::{Arc, Mutex}};
    /// # trait Trait {}
    /// # #[coi(provides dyn Trait + Send + Sync with Impl)]
    /// # struct Impl;
    /// # impl Trait for Impl {}
    /// # fn the_test() -> Result<()> {
    /// let container = container! {
    ///     trait1 => ImplProvider; singleton
    /// };
    ///
    /// let instance_1 = container.resolve::<dyn Trait + Send + Sync>("trait1")?;
    /// let instance_2 = container.resolve::<dyn Trait + Send + Sync>("trait1")?;
    ///
    /// assert_eq!(
    ///     instance_1.deref() as &dyn Trait as *const _,
    ///     instance_2.deref() as &dyn Trait as *const _
    /// );
    /// {
    ///     let scoped = container.scoped();
    ///     let instance_3 = scoped.resolve::<dyn Trait + Send + Sync>("trait1")?;
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
    /// `Container` will construct a new instance of `T` for each scope
    /// container created through `Container::scoped`.
    ///
    /// # Example
    /// ```rust
    /// # use coi::{coi, container, Result};
    /// # use std::{ops::Deref, sync::{Arc, Mutex}};
    /// # trait Trait {}
    /// # #[coi(provides dyn Trait + Send + Sync with Impl)]
    /// # struct Impl;
    /// # impl Trait for Impl {}
    /// # fn the_test() -> Result<()> {
    /// let container = container! {
    ///     trait1 => ImplProvider; scoped
    /// };
    ///
    /// // Every instance resolved within the same scope will be the same instance.
    /// let instance_1 = container.resolve::<dyn Trait + Send + Sync>("trait1")?;
    /// let instance_2 = container.resolve::<dyn Trait + Send + Sync>("trait1")?;
    /// assert_eq!(
    ///     instance_1.deref() as &dyn Trait as *const _,
    ///     instance_2.deref() as &dyn Trait as *const _
    /// );
    /// {
    ///     let scoped = container.scoped();
    ///     let instance_3 = scoped.resolve::<dyn Trait + Send + Sync>("trait1")?;
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
    /// `Container` will construct a new instance of `T` for every invocation
    /// of `Container::resolve`.
    ///
    /// # Example
    /// ```rust
    /// # use coi::{coi, container, Result};
    /// # use std::ops::Deref;
    /// # trait Trait {}
    /// # #[coi(provides dyn Trait + Send + Sync with Impl)]
    /// # struct Impl;
    /// # impl Trait for Impl {}
    /// # fn the_test() -> Result<()> {
    /// let mut container = container! {
    ///     // same as trait1 => ImplProvider.transient
    ///     trait1 => ImplProvider
    /// };
    ///
    /// let instance_1 = container.resolve::<dyn Trait + Send + Sync>("trait1")?;
    /// let instance_2 = container.resolve::<dyn Trait + Send + Sync>("trait1")?;
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
}
