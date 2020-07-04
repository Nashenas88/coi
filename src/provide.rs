use crate::container::Container;
use crate::Result;
use std::sync::Arc;

/// A trait to manage the construction of an injectable trait or struct.
pub trait Provide {
    /// The type that this provider will produce when resolved from a [`Container`].
    ///
    /// [`Container`]: struct.Container.html
    type Output: Send + Sync + ?Sized;

    /// Only intended to be used internally
    fn provide(&self, container: &Container) -> Result<Arc<Self::Output>>;
}

impl<T, F> Provide for F
where
    F: Fn(&Container) -> Result<Arc<T>>,
    T: Send + Sync + ?Sized,
{
    type Output = T;

    fn provide(&self, container: &Container) -> Result<Arc<Self::Output>> {
        self(container)
    }
}

impl<T> Provide for dyn Fn(&Container) -> Result<Arc<T>>
where
    T: Send + Sync + ?Sized,
{
    type Output = T;

    fn provide(&self, container: &Container) -> Result<Arc<Self::Output>> {
        self(container)
    }
}

/// Return list of dependencies
#[cfg(feature = "debug")]
#[cfg_attr(docsrs, doc(cfg(feature = "debug")))]
pub trait Dependencies: Provide {
    /// Return list of dependencies
    fn dependencies(&self) -> &'static [&'static str] {
        &["unknown"]
    }
}

#[cfg(feature = "debug")]
#[cfg_attr(docsrs, doc(cfg(feature = "debug")))]
impl<T, F> Dependencies for (&'static [&'static str], F)
where
    Self: Provide<Output = T>,
    F: Fn(&Container) -> Result<Arc<T>>,
    T: Send + Sync + ?Sized,
{
    fn dependencies(&self) -> &'static [&'static str] {
        self.0
    }
}

#[cfg(feature = "debug")]
#[cfg_attr(docsrs, doc(cfg(feature = "debug")))]
impl<T> Dependencies
    for (
        &'static [&'static str],
        dyn Fn(&Container) -> Result<Arc<T>>,
    )
where
    Self: Provide<Output = T>,
    T: Send + Sync + ?Sized,
{
    fn dependencies(&self) -> &'static [&'static str] {
        self.0
    }
}
