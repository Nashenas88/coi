use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

pub enum Error {
    /// This key was not found in the container. Either the requested resource was never registered
    /// with this container, or there is a typo in the register or resolve calls.
    KeyNotFound(String),
    /// The requested key was found in the container, but its type did not match the requested type.
    TypeMismatch(String),
    /// Wrapper around errors produced by `Provider`s.
    Inner(Box<dyn std::error::Error + Send + Sync + 'static>),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Container {
    provider_map: HashMap<String, Arc<dyn Any + Send + Sync + 'static>>,
}

impl Container {
    /// Construct or lookup a previously constructed object of type `T` with key `key`.
    pub async fn resolve<T>(&self, key: &str) -> Result<T>
    where
        T: Send + Sync + 'static,
    {
        let any_provider = self
            .provider_map
            .get(key)
            .ok_or_else(|| Error::KeyNotFound(key.to_owned()))?;
        let provider = any_provider
            .downcast_ref::<Arc<dyn Provider<Output = T> + Send + Sync + 'static>>()
            .ok_or_else(|| Error::TypeMismatch(key.to_owned()))?;
        // FIXME(pfaria) memoize results when singleton registration is introduced
        provider.provide(self).await
    }
}

/// A builder used to construct a `Container`
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
        T: Send + Sync + 'static,
        P: Provider<Output = T> + Send + Sync + 'static,
    {
        let key = key.into();
        let provider = Arc::new(provider) as Arc<dyn Provider<Output = T> + Send + Sync + 'static>;
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
pub trait Provider {
    /// The type that this provider is intended to produce
    type Output: Send + Sync + 'static;

    /// Only intended to be used internally
    async fn provide(&self, container: &Container) -> Result<Self::Output>;
}
