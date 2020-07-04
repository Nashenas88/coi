use crate::registration::RegistrationKind;
use std::fmt::{self, Debug};

/// Possible errors generated when running [`Container::analyze`].
///
/// [`Container::analyze`]: struct.Container.html#method.analyze
#[cfg(feature = "debug")]
#[cfg_attr(docsrs, doc(cfg(feature = "debug")))]
#[derive(Debug, thiserror::Error)]
pub enum AnalysisError {
    // FIXME(pfaria), it would be better if we could trace the
    // entire cycle and store a Vec<String> here. Might require
    // manually calling petgraph::visit::depth_first_search
    /// There is a cyclic dependency within the container
    #[error("Cycle detected at node `{0}`")]
    Cycle(&'static str),
    /// There is a missing dependency. Param 0 depends on Param 1, and Param 1 is missing.
    #[error("Node `{0}` depends on `{1}`, the latter of which is not registered")]
    Missing(&'static str, &'static str),
}

#[cfg(feature = "debug")]
#[derive(Clone, Default)]
pub(crate) struct AnalysisNode {
    pub(crate) registration: Option<RegistrationKind>,
    pub(crate) id: &'static str,
}

#[cfg(feature = "debug")]
impl fmt::Display for AnalysisNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.registration {
            Some(reg) => match reg {
                RegistrationKind::Transient => write!(f, "Transient - {}", self.id),
                RegistrationKind::Singleton => write!(f, "Singleton - {}", self.id),
                RegistrationKind::Scoped => write!(f, "Scoped - {}", self.id),
            },
            None => write!(f, "MISSING - {}", self.id),
        }
    }
}
