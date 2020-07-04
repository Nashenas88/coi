use crate::container::Container;
use crate::registration::RegistrationKind;
use crate::{ContainerKey, Error, Result};
use std::any::Any;
use std::cell::UnsafeCell;
#[cfg(feature = "debug")]
use std::fmt::{self, Debug};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Condvar, Mutex};

pub(crate) struct Resolvable {
    variant: AtomicU8,
    cond: Arc<(Mutex<bool>, Condvar)>,
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
            cond: Arc::new((Mutex::new(false), Condvar::new())),
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
        if self
            .variant
            .compare_and_swap(UNRESOLVED, BUILDING, Ordering::Relaxed)
            == UNRESOLVED
        {
            // we now own resolution
            self.resolve_inner(key, kind, container)
        } else {
            loop {
                match self.variant.load(Ordering::Relaxed) {
                    // Another thread had an error resolving, try to take over
                    UNRESOLVED => {
                        if self
                            .variant
                            .compare_and_swap(UNRESOLVED, BUILDING, Ordering::Relaxed)
                            == UNRESOLVED
                        {
                            // we now own resolution
                            return self.resolve_inner(key, kind, container);
                        }
                    }
                    BUILDING => {
                        let mut is_building = self.cond.0.lock().unwrap();
                        while *is_building {
                            is_building = self.cond.1.wait(is_building).unwrap();
                        }
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
    }

    // TODO maybe inline?
    fn resolve_inner<T>(
        &self,
        key: ContainerKey,
        kind: RegistrationKind,
        container: &Container,
    ) -> Result<Arc<T>>
    where
        T: Send + Sync + ?Sized + 'static,
    {
        {
            let mut is_building = self.cond.0.lock().unwrap();
            *is_building = true;
        }
        let resolved = match container.resolve_inner::<T>(key, kind) {
            Ok(resolved) => resolved,
            Err(e) => {
                let val = self
                    .variant
                    .compare_and_swap(BUILDING, UNRESOLVED, Ordering::SeqCst);
                assert_eq!(val, BUILDING);
                let mut is_building = self.cond.0.lock().unwrap();
                *is_building = false;
                self.cond.1.notify_all();
                return Err(e);
            }
        };
        unsafe {
            *self.item.get() = Some(Box::new(resolved.clone()) as Box<dyn Any + Send + Sync>)
        };
        let val = self
            .variant
            .compare_and_swap(BUILDING, RESOLVED, Ordering::SeqCst);
        assert_eq!(val, BUILDING);
        let mut is_building = self.cond.0.lock().unwrap();
        *is_building = false;
        self.cond.1.notify_all();
        Ok(resolved)
    }
}

unsafe impl Sync for Resolvable {}
