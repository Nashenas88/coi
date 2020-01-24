//! (TODO)

// re-export coi for convenience
pub use coi;

use actix_web::{
    dev::Payload,
    error::{Error, ErrorInternalServerError, Result},
    FromRequest, HttpRequest,
};
use coi::{Container, Inject};
use futures::future::{err, ok, ready, Ready};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

pub trait ContainerKey<T>
where
    T: Inject + ?Sized,
{
    const KEY: &'static str;
}

pub struct Injected<T, K>(pub T, pub PhantomData<K>);

impl<T, K> Injected<T, K> {
    pub fn new(injected: T) -> Self {
        Self(injected, PhantomData)
    }
}

impl<T, K> FromRequest for Injected<Arc<T>, K>
where
    T: Inject + ?Sized,
    K: ContainerKey<T>,
{
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.app_data::<Arc<Mutex<Container>>>() {
            Some(container) => ready(
                Container::scopable(Arc::clone(container))
                    .scoped()
                    .resolve::<T>(K::KEY)
                    .map(Injected::new)
                    .map_err(|e| {
                        log::error!("{}", e);
                        ErrorInternalServerError("huh")
                    }),
            ),
            None => {
                log::error!("Container not registered");
                err(ErrorInternalServerError("huh2"))
            }
        }
    }
}

macro_rules! injected_tuples {
    ($(($T:ident, $K:ident)),+) => {
        impl<$($T, $K),+> FromRequest for Injected<($(Arc<$T>),+), ($($K),+)>
        where $(
            $T: Inject + ?Sized,
            $K: ContainerKey<$T>,
        )+
        {
            type Error = Error;
            type Future = Ready<Result<Self, Self::Error>>;
            type Config = ();

            fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
                match req.app_data::<Arc<Mutex<Container>>>() {
                    Some(container) => {
                        let mut container = Container::scopable(Arc::clone(&container)).scoped();
                        ok(Injected::new(($(
                            {
                                let resolved = container
                                    .resolve::<$T>(<$K as ContainerKey<$T>>::KEY)
                                    .map_err(ErrorInternalServerError);
                                match resolved {
                                    Ok(r) => r,
                                    Err(e) => return err(e),
                                }
                            },
                        )+)))
                    },
                    None => err(ErrorInternalServerError("Container not registered"))
                }
            }
        }
    }
}

injected_tuples!((TA, KA), (TB, KB));
injected_tuples!((TA, KA), (TB, KB), (TC, KC));
injected_tuples!((TA, KA), (TB, KB), (TC, KC), (TD, KD));
injected_tuples!((TA, KA), (TB, KB), (TC, KC), (TD, KD), (TE, KE));
injected_tuples!((TA, KA), (TB, KB), (TC, KC), (TD, KD), (TE, KE), (TF, KF));
injected_tuples!(
    (TA, KA),
    (TB, KB),
    (TC, KC),
    (TD, KD),
    (TE, KE),
    (TF, KF),
    (TG, KG)
);
injected_tuples!(
    (TA, KA),
    (TB, KB),
    (TC, KC),
    (TD, KD),
    (TE, KE),
    (TF, KF),
    (TG, KG),
    (TH, KH)
);
injected_tuples!(
    (TA, KA),
    (TB, KB),
    (TC, KC),
    (TD, KD),
    (TE, KE),
    (TF, KF),
    (TG, KG),
    (TH, KH),
    (TI, KI)
);
injected_tuples!(
    (TA, KA),
    (TB, KB),
    (TC, KC),
    (TD, KD),
    (TE, KE),
    (TF, KF),
    (TG, KG),
    (TH, KH),
    (TI, KI),
    (TJ, KJ)
);
injected_tuples!(
    (TA, KA),
    (TB, KB),
    (TC, KC),
    (TD, KD),
    (TE, KE),
    (TF, KF),
    (TG, KG),
    (TH, KH),
    (TI, KI),
    (TJ, KJ),
    (TK, KK)
);
