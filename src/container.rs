use crate::provide::Provide;
use crate::registration::RegistrationKind;
use crate::resolvable::Resolvable;
use crate::{ContainerKey, Error, Result};
use rustc_hash::FxHashMap;
use std::any::Any;
use std::sync::Arc;

#[cfg(feature = "debug")]
use {
    crate::debug::{AnalysisError, AnalysisNode},
    crate::provide::Dependencies,
    petgraph::{
        algo::toposort,
        graph::{DiGraph, NodeIndex},
    },
    std::fmt::Debug,
};

type ProviderMap = FxHashMap<ContainerKey, Box<dyn Any + Send + Sync>>;
type ResolvedMap = FxHashMap<ContainerKey, Resolvable>;

/// A struct that manages all injected types.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Container {
    kind_lookup: Arc<FxHashMap<ContainerKey, RegistrationKind>>,
    singleton_provider_map: Arc<ProviderMap>,
    scoped_provider_map: Arc<ProviderMap>,
    transient_provider_map: Arc<ProviderMap>,
    singleton_resolved_map: Arc<ResolvedMap>,
    scoped_resolved_map: ResolvedMap,
    #[cfg(feature = "debug")]
    dependency_map: FxHashMap<ContainerKey, &'static [ContainerKey]>,
}

impl Container {
    /// Resolve an `Arc<T>` whose provider was previously registered with `key`.
    pub fn resolve<T>(&self, key: ContainerKey) -> Result<Arc<T>>
    where
        T: Send + Sync + ?Sized + 'static,
    {
        let &kind = self
            .kind_lookup
            .get(key)
            .ok_or_else(|| Error::KeyNotFound(key))?;
        match kind {
            RegistrationKind::Singleton => self
                .singleton_resolved_map
                .get(key)
                .unwrap()
                .resolve(key, kind, self),
            RegistrationKind::Scoped => self
                .scoped_resolved_map
                .get(key)
                .unwrap()
                .resolve(key, kind, self),
            RegistrationKind::Transient => self
                .transient_provider_map
                .get(key)
                .unwrap()
                .downcast_ref::<Box<dyn Provide<Output = T> + Send + Sync>>()
                .map(|p| p.provide(self))
                .unwrap_or_else(|| Err(Error::TypeMismatch(key))),
        }
    }

    pub(crate) fn resolve_inner<T>(
        &self,
        key: ContainerKey,
        kind: RegistrationKind,
    ) -> Result<Arc<T>>
    where
        T: Send + Sync + ?Sized + 'static,
    {
        match kind {
            RegistrationKind::Singleton => self
                .singleton_provider_map
                .get(key)
                .unwrap()
                .downcast_ref::<Box<dyn Provide<Output = T> + Send + Sync>>()
                .map(|p| p.provide(self))
                .unwrap_or_else(|| Err(Error::TypeMismatch(key))),
            RegistrationKind::Scoped => self
                .scoped_provider_map
                .get(key)
                .unwrap()
                .downcast_ref::<Box<dyn Provide<Output = T> + Send + Sync>>()
                .map(|p| p.provide(self))
                .unwrap_or_else(|| Err(Error::TypeMismatch(key))),
            RegistrationKind::Transient => unreachable!(),
        }
    }

    /// Produce a child container the clears the scoped resolutions. Singleton instances are
    /// still shared with the parent `Container`.
    pub fn scoped(&self) -> Self {
        Self {
            kind_lookup: self.kind_lookup.clone(),
            singleton_provider_map: self.singleton_provider_map.clone(),
            scoped_provider_map: self.scoped_provider_map.clone(),
            transient_provider_map: self.transient_provider_map.clone(),
            singleton_resolved_map: self.singleton_resolved_map.clone(),
            scoped_resolved_map: self
                .scoped_resolved_map
                .iter()
                .map(|(k, _)| (*k, Resolvable::new()))
                .collect(),
            #[cfg(feature = "debug")]
            dependency_map: self.dependency_map.clone(),
        }
    }

    #[cfg(feature = "debug")]
    fn dependency_graph(&self) -> DiGraph<AnalysisNode, AnalysisNode> {
        let mut graph = DiGraph::<AnalysisNode, AnalysisNode>::new();
        let mut key_to_node = self
            .dependency_map
            .iter()
            .map(|(k, _)| -> (&'static str, NodeIndex) {
                let kind = self.kind_lookup[k];
                let n = graph.add_node(AnalysisNode {
                    registration: Some(kind),
                    id: k,
                });
                (k, n)
            })
            .collect::<FxHashMap<&str, _>>();
        for (k, deps) in &self.dependency_map {
            let kn = key_to_node[*k];
            let edges = deps
                .iter()
                .map(|dep| {
                    let vn = match key_to_node.get(dep) {
                        Some(vn) => *vn,
                        None => {
                            let vn = graph.add_node(AnalysisNode {
                                registration: None,
                                id: dep,
                            });
                            key_to_node.insert(dep, vn);
                            key_to_node[dep]
                        }
                    };
                    (kn, vn)
                })
                .collect::<Vec<_>>();
            graph.extend_with_edges(&edges[..]);
        }

        graph
    }

    // FIXME(pfaria): Add analysis on singleton registrations that depend on
    // non-singleton registration.
    /// Run an analysis on a container and return any issues detected.
    /// Current analysis performed:
    /// - Missing dependencies
    /// - Cyclic dependencies
    #[cfg(feature = "debug")]
    #[cfg_attr(docsrs, doc(cfg(feature = "debug")))]
    pub fn analyze(&self) -> std::result::Result<(), Vec<AnalysisError>> {
        use petgraph::Direction;
        let graph = self.dependency_graph();
        let mut errors = graph
            .node_indices()
            .filter(|i| graph[*i].registration.is_none())
            .map(|i| {
                let to = &graph[i].id;
                graph
                    .neighbors_directed(i, Direction::Incoming)
                    .map(|from| AnalysisError::Missing(graph[from].id, to))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        // Do any cycles exist?
        if let Err(cycle) = toposort(&graph, None) {
            errors.push(AnalysisError::Cycle(graph[cycle.node_id()].id));
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }

    /// Produces a dot format output that can be processed by the [graphviz] [`dot` (pdf)]
    /// program to generate a graphical representation of the dependency graph.
    ///
    /// [graphviz]: http://graphviz.org/
    /// [`dot` (pdf)]: https://graphviz.gitlab.io/_pages/pdf/dotguide.pdf
    #[cfg(feature = "debug")]
    #[cfg_attr(docsrs, doc(cfg(feature = "debug")))]
    pub fn dot_graph(&self) -> String {
        use petgraph::dot::{Config, Dot};
        let graph = self.dependency_graph();
        format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]))
    }
}

/// A builder used to construct a `Container`.
#[derive(Default)]
pub struct ContainerBuilder {
    kind_lookup: FxHashMap<ContainerKey, RegistrationKind>,
    singleton_map: ProviderMap,
    scoped_map: ProviderMap,
    transient_map: ProviderMap,
    #[cfg(feature = "debug")]
    dependency_map: FxHashMap<ContainerKey, &'static [ContainerKey]>,
}

impl ContainerBuilder {
    /// Constructor for `ContainerBuilder`.
    pub fn new() -> Self {
        Self {
            kind_lookup: FxHashMap::default(),
            singleton_map: FxHashMap::default(),
            scoped_map: FxHashMap::default(),
            transient_map: FxHashMap::default(),
            #[cfg(feature = "debug")]
            dependency_map: FxHashMap::default(),
        }
    }

    /// Register a provider for `T` with identifier `key`.
    #[inline]
    pub fn register<P>(self, key: ContainerKey, provider: P) -> Self
    where
        P: Provide + Send + Sync + 'static,
    {
        self.register_as(key, provider, RegistrationKind::Transient)
    }

    /// Register a provider for `T` with identifier `key`, while also specifying the resolution
    /// behavior.
    pub fn register_as<P>(mut self, key: ContainerKey, provider: P, kind: RegistrationKind) -> Self
    where
        P: Provide + Send + Sync + 'static,
    {
        self.kind_lookup.insert(key, kind);
        let provider = Box::new(provider) as Box<dyn Provide<Output = P::Output> + Send + Sync>;
        let provider = Box::new(provider) as Box<dyn Any + Send + Sync>;
        match kind {
            RegistrationKind::Singleton => {
                self.singleton_map.insert(key, provider);
            }
            RegistrationKind::Scoped => {
                self.scoped_map.insert(key, provider);
            }
            RegistrationKind::Transient => {
                self.transient_map.insert(key, provider);
            }
        }

        self
    }

    /// Track a dependency from identifier `key` to the depenendencies needed by `provider`.
    #[cfg(feature = "debug")]
    #[cfg_attr(docsrs, doc(cfg(feature = "debug")))]
    pub fn track_dependencies<D>(mut self, key: ContainerKey, provider: D) -> Self
    where
        D: Dependencies,
    {
        self.dependency_map.insert(key, provider.dependencies());
        self
    }

    /// Consume this builder to produce a `Container`.
    pub fn build(self) -> Container {
        let singleton_resolved_map = Arc::new(
            self.singleton_map
                .iter()
                .map(|(k, _)| (*k, Resolvable::new()))
                .collect(),
        );
        let scoped_resolved_map = self
            .scoped_map
            .iter()
            .map(|(k, _)| (*k, Resolvable::new()))
            .collect();

        Container {
            kind_lookup: Arc::new(self.kind_lookup),
            singleton_provider_map: Arc::new(self.singleton_map),
            scoped_provider_map: Arc::new(self.scoped_map),
            transient_provider_map: Arc::new(self.transient_map),
            singleton_resolved_map,
            scoped_resolved_map,
            #[cfg(feature = "debug")]
            dependency_map: self.dependency_map,
        }
    }
}
