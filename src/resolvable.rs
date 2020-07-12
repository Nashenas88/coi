use crate::container::Container;
use crate::registration::RegistrationKind;
use crate::{ContainerKey, Error, Result};
use std::any::Any;
use std::cell::UnsafeCell;
#[cfg(feature = "debug")]
use std::fmt::{self, Debug};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use parking_lot::{RwLock, RwLockWriteGuard};

pub(crate) struct Resolvable {
    variant: AtomicU8,
    building_lock: RwLock<()>,
    item: UnsafeCell<Option<Box<dyn Any + Send + Sync>>>,
}

#[cfg(feature = "debug")]
impl Debug for Resolvable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.variant.load(Ordering::Relaxed) {
                UNRESOLVED => {
                    "unresolved"
                }
                BUILDING => {
                    "building"
                }
                RESOLVED => {
                    "resolved"
                }
                _ => unreachable!(),
            }
        )
    }
}

const UNRESOLVED: u8 = 0;
const BUILDING: u8 = 1;
const RESOLVED: u8 = 3;

impl Resolvable {
    pub(crate) fn new() -> Self {
        Self {
            variant: AtomicU8::new(UNRESOLVED),
            building_lock: RwLock::new(()),
            item: UnsafeCell::new(None),
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
        let state = self.variant.load(Ordering::Acquire);
        if state == UNRESOLVED {
            let write = self.building_lock.write();
            if self
                .variant
                .compare_and_swap(UNRESOLVED, BUILDING, Ordering::AcqRel)
                == UNRESOLVED
            {
                // we now own resolution
                return self.resolve_inner(key, kind, container, write);
            }
        }

        loop {
            match self.variant.load(Ordering::Relaxed) {
                // Another thread had an error resolving, try to take over
                UNRESOLVED => {
                    let write = self.building_lock.write();
                    if self
                        .variant
                        .compare_and_swap(UNRESOLVED, BUILDING, Ordering::AcqRel)
                        == UNRESOLVED
                    {
                        // we now own resolution
                        return self.resolve_inner(key, kind, container, write);
                    }
                }
                BUILDING => {
                    // block this thread while building is in progress
                    let _read = self.building_lock.read();
                }
                RESOLVED => {
                    break;
                }
                _ => unreachable!(),
            }
        }

        let any = Option::as_ref(unsafe { &*self.item.get() }).unwrap();
        match any.downcast_ref::<Arc<T>>() {
            Some(val) => Ok(val.clone()),
            None => Err(Error::KeyNotFound(key)),
        }
    }

    // TODO maybe inline?
    fn resolve_inner<T>(
        &self,
        key: ContainerKey,
        kind: RegistrationKind,
        container: &Container,
        // This lock keeps other threads interested in this key blocked while
        // building is in progress
        _write_lock: RwLockWriteGuard<()>,
    ) -> Result<Arc<T>>
    where
        T: Send + Sync + ?Sized + 'static,
    {
        let resolved = match container.resolve_inner::<T>(key, kind) {
            Ok(resolved) => resolved,
            err @ Err(_) => {
                let val = self
                    .variant
                    .compare_and_swap(BUILDING, UNRESOLVED, Ordering::SeqCst);
                assert_eq!(val, BUILDING);
                return err;
            }
        };

        unsafe {
            *self.item.get() = Some(Box::new(resolved.clone()) as Box<dyn Any + Send + Sync>)
        };

        let val = self
            .variant
            .compare_and_swap(BUILDING, RESOLVED, Ordering::SeqCst);
        assert_eq!(val, BUILDING);
        Ok(resolved)
    }
}

unsafe impl Sync for Resolvable {}
