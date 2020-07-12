use crate::container::Container;
use crate::registration::RegistrationKind;
use crate::{ContainerKey, Error, Result};
use parking_lot::{RwLock, RwLockWriteGuard};
use std::any::Any;
#[cfg(feature = "debug")]
use std::fmt::{self, Debug};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

pub(crate) struct Resolvable {
    state: AtomicU8,
    item: Arc<RwLock<Option<Box<dyn Any + Send + Sync>>>>,
}

#[cfg(feature = "debug")]
impl Debug for Resolvable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.state.load(Ordering::Relaxed) {
                UNRESOLVED => "unresolved",
                BUILDING => "building",
                RESOLVED => "resolved",
                _ => unreachable!(),
            }
        )
    }
}

const UNRESOLVED: u8 = 0;
const BUILDING: u8 = 1;
const RESOLVED: u8 = 2;

impl Resolvable {
    pub(crate) fn new() -> Self {
        Self {
            state: AtomicU8::new(UNRESOLVED),
            item: Arc::new(RwLock::new(None)),
        }
    }

    pub(crate) fn resolve<T>(
        &self,
        key: ContainerKey,
        kind: RegistrationKind,
        container: &Container,
    ) -> Result<Arc<T>>
    where
        T: Send + Sync + ?Sized + 'static,
    {
        let state = self.state.load(Ordering::Acquire);
        if state == UNRESOLVED {
            let item = self.item.write();
            if self
                .state
                .compare_and_swap(UNRESOLVED, BUILDING, Ordering::Relaxed)
                == UNRESOLVED
            {
                return self.resolve_inner(key, kind, container, item);
            }
        }

        loop {
            match self.state.load(Ordering::Acquire) {
                UNRESOLVED => {
                    let item = self.item.write();
                    if self
                        .state
                        .compare_and_swap(UNRESOLVED, BUILDING, Ordering::Relaxed)
                        == UNRESOLVED
                    {
                        return self.resolve_inner(key, kind, container, item);
                    }
                }
                BUILDING => {
                    // blocks this thread while building
                    let _item = self.item.read();
                }
                RESOLVED => {
                    break;
                }
                _ => unreachable!(),
            }
        }

        let item = self.item.read();
        let any = Option::as_ref(&*item).unwrap();
        match any.downcast_ref::<Arc<T>>() {
            Some(val) => Ok(val.clone()),
            None => Err(Error::TypeMismatch(key)),
        }
    }

    // TODO maybe inline?
    fn resolve_inner<T>(
        &self,
        key: ContainerKey,
        kind: RegistrationKind,
        container: &Container,
        mut item: RwLockWriteGuard<Option<Box<dyn Any + Send + Sync>>>,
    ) -> Result<Arc<T>>
    where
        T: Send + Sync + ?Sized + 'static,
    {
        let resolved = match container.resolve_inner::<T>(key, kind) {
            Ok(resolved) => resolved,
            err @ Err(_) => {
                let val = self
                    .state
                    .compare_and_swap(BUILDING, UNRESOLVED, Ordering::SeqCst);
                assert_eq!(val, BUILDING);
                return err;
            }
        };

        *item = Some(Box::new(resolved.clone()) as Box<dyn Any + Send + Sync>);
        let val = self
            .state
            .compare_and_swap(BUILDING, RESOLVED, Ordering::SeqCst);
        assert_eq!(val, BUILDING);
        Ok(resolved)
    }
}

// unsafe impl Sync for Resolvable {}
