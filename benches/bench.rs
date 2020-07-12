use criterion::{criterion_group, criterion_main};

mod deep;
mod simple;
mod threads;
mod wide;

criterion_group!(simple, simple::a_simple_resolve);
criterion_group!(threads, threads::run_15_background_threads_while_resolving);

criterion_group!(
    deeply_nested,
    deep::deeply_nested_transient_dependencies,
    deep::deeply_nested_singleton_dependencies,
    deep::deeply_nested_scoped_dependencies
);
criterion_group!(
    scoped_deeply_nested,
    deep::scoped_container_deeply_nested_transient_dependencies,
    deep::scoped_container_deeply_nested_singleton_dependencies,
    deep::scoped_container_deeply_nested_scoped_dependencies
);
criterion_group!(
    double_scoped_deeply_nested,
    deep::doubly_scoped_container_deeply_nested_transient_dependencies,
    deep::doubly_scoped_container_deeply_nested_singleton_dependencies,
    deep::doubly_scoped_container_deeply_nested_scoped_dependencies
);

criterion_group!(
    wide,
    wide::wide_transient_dependencies,
    wide::wide_singleton_dependencies,
    wide::wide_scoped_dependencies
);
criterion_group!(
    scoped_wide,
    wide::scoped_container_wide_transient_dependencies,
    wide::scoped_container_wide_singleton_dependencies,
    wide::scoped_container_wide_scoped_dependencies
);
criterion_group!(
    double_scoped_wide,
    wide::doubly_scoped_container_wide_transient_dependencies,
    wide::doubly_scoped_container_wide_singleton_dependencies,
    wide::doubly_scoped_container_wide_scoped_dependencies
);

criterion_main!(
    simple,
    threads,
    deeply_nested,
    scoped_deeply_nested,
    double_scoped_deeply_nested,
    wide,
    scoped_wide,
    double_scoped_wide
);