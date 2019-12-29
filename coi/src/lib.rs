use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::sync::Arc;
pub use coi_derive::*;

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

pub trait Injectable: Send + Sync + 'static {}

impl<T: Injectable + ?Sized> Injectable for Arc<T> {}

#[derive(Clone)]
pub struct Container {
    provider_map: HashMap<String, Arc<dyn Any + Send + Sync + 'static>>,
}

impl Container {
    /// Construct or lookup a previously constructed object of type `T` with key `key`.
    pub async fn resolve<T>(&self, key: &str) -> Result<T>
    where
        T: Injectable,
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
        T: Injectable,
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
    type Output: Send + Sync + 'static;

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