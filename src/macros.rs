/// A macro to simplify building of `Container`s.
///
/// It takes a list of key-value pairs, where the keys are converted to string
/// keys, and the values are converted into registrations. Transient, singleton
/// and scoped registrations are possible, with transient being the default:
/// ```rust
/// use coi::{coi, container};
///
/// trait Dep {}
///
/// #[coi(provides dyn Dep + Send + Sync with Impl)]
/// struct Impl;
///
/// impl Dep for Impl {}
///
/// let mut container = container! {
///     dep => ImplProvider,
///     transient_dep => ImplProvider; transient,
///     singleton_dep => ImplProvider; singleton,
///     scoped_dep => ImplProvider; scoped
/// };
/// ```
///
/// For details on how each registration works, see [`coi::Registration`]
///
/// [`coi::Registration`]: enum.Registration.html
#[macro_export]
macro_rules! container {
    (@registration scoped) => {
        $crate::RegistrationKind::Scoped
    };
    (@registration singleton) => {
        $crate::RegistrationKind::Singleton
    };
    (@registration $(transient)?) => {
        $crate::RegistrationKind::Transient
    };
    ($($key:ident => $provider:expr $(; $call:ident)?),+ $(,)?) => {
        {
            let mut builder = $crate::ContainerBuilder::new();
            $(
                builder = builder.register_as(stringify!($key), $provider, container!(@registration $($call)?));
                $crate::__container_track!(builder $key => $provider);
            )+
            builder.build()
        }
    }
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "debug"))]
macro_rules! __container_track {
    ($builder:ident $key:ident => $provider:expr) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "debug")]
macro_rules! __container_track {
    ($builder:ident $key:ident => $provider:expr) => {
        $builder = $builder.track_dependencies(stringify!($key), $provider);
    };
}

/// Helper macro to ease use of "debug" feature when providing closures
#[macro_export]
macro_rules! provide_closure {
    // actual macro
    ($($move:ident)? |$($arg:ident: Arc<$ty:ty>),* $(,)?| $(-> $res:ty)? $block:block) => {
        {
            $crate::__provide_closure_impl!($($move)? |$($arg: $ty,)*| $(-> $res)? $block)
        }
    };
    // handle case of missing argument types
    ($($move:ident)? |$($arg:ident),* $(,)?| $(-> $res:ty)? $block:block) => {
        compile_error!("this macro requires closure arguments to have explicitly defined parameter types")
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "debug"))]
macro_rules! __provide_closure_impl {
    ($($move:ident)? |$($arg:ident: $ty:ty,)*| $(-> $res:ty)? $block:block) => {
        $($move)? |_container: &$crate::Container| $(-> $res)? {
            $(let $arg = _container.resolve::<$ty>(stringify!($arg))?;)*
            $block
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "debug")]
macro_rules! __provide_closure_impl {
    ($($move:ident)? |$($arg:ident: $ty:ty,)*| $(-> $res:ty)? $block:block) => {
        (
            &[$(stringify!($arg),)*],
            $($move)? |_container: &$crate::Container| $(-> $res)? {
                $(let $arg = _container.resolve::<$ty>(stringify!($arg))?;)*
                $block
            }
        )
    };
}
