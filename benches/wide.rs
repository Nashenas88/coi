use coi::{coi, container};
use criterion::Criterion;
use std::sync::Arc;

macro_rules! make_wide_container {
    ($($scope_type:ident)?) => {
        container! {
            w1 => W1Provider$(;$scope_type)?,
            w2 => W2Provider$(;$scope_type)?,
            w3 => W3Provider$(;$scope_type)?,
            w4 => W4Provider$(;$scope_type)?,
            w5 => W5Provider$(;$scope_type)?,
            w6 => W6Provider$(;$scope_type)?,
            w7 => W7Provider$(;$scope_type)?,
            w8 => W8Provider$(;$scope_type)?,
            w9 => W9Provider$(;$scope_type)?,
            w10 => W10Provider$(;$scope_type)?,
            w11 => W11Provider$(;$scope_type)?,
            w12 => W12Provider$(;$scope_type)?,
            w13 => W13Provider$(;$scope_type)?,
            w14 => W14Provider$(;$scope_type)?,
            w15 => W15Provider$(;$scope_type)?,
            w16 => W16Provider$(;$scope_type)?,
            w17 => W17Provider$(;$scope_type)?,
            w18 => W18Provider$(;$scope_type)?,
            w19 => W19Provider$(;$scope_type)?,
            w20 => W20Provider$(;$scope_type)?,
            w21 => W21Provider$(;$scope_type)?,
            w22 => W22Provider$(;$scope_type)?,
            w23 => W23Provider$(;$scope_type)?,
            w24 => W24Provider$(;$scope_type)?,
            w25 => W25Provider$(;$scope_type)?,
            w26 => W26Provider$(;$scope_type)?,
            w27 => W27Provider$(;$scope_type)?,
            w28 => W28Provider$(;$scope_type)?,
            w29 => W29Provider$(;$scope_type)?,
            w30 => W30Provider$(;$scope_type)?,
            w31 => W31Provider$(;$scope_type)?,
            w32 => W32Provider$(;$scope_type)?,
            w33 => W33Provider$(;$scope_type)?,
            w34 => W34Provider$(;$scope_type)?,
            w35 => W35Provider$(;$scope_type)?,
            w36 => W36Provider$(;$scope_type)?,
            w37 => W37Provider$(;$scope_type)?,
            w38 => W38Provider$(;$scope_type)?,
            w39 => W39Provider$(;$scope_type)?,
            w40 => W40Provider$(;$scope_type)?,
            w41 => W41Provider$(;$scope_type)?,
            w42 => W42Provider$(;$scope_type)?,
            w43 => W43Provider$(;$scope_type)?,
            w44 => W44Provider$(;$scope_type)?,
            w45 => W45Provider$(;$scope_type)?,
            w46 => W46Provider$(;$scope_type)?,
            w47 => W47Provider$(;$scope_type)?,
            w48 => W48Provider$(;$scope_type)?,
            w49 => W49Provider$(;$scope_type)?,
            w50 => W50Provider$(;$scope_type)?,
            w51 => W51Provider$(;$scope_type)?,
            w52 => W52Provider$(;$scope_type)?,
            w53 => W53Provider$(;$scope_type)?,
            w54 => W54Provider$(;$scope_type)?,
            w55 => W55Provider$(;$scope_type)?,
            w56 => W56Provider$(;$scope_type)?,
            w57 => W57Provider$(;$scope_type)?,
            w58 => W58Provider$(;$scope_type)?,
            w59 => W59Provider$(;$scope_type)?,
            w60 => W60Provider$(;$scope_type)?,
            w61 => W61Provider$(;$scope_type)?,
            w62 => W62Provider$(;$scope_type)?,
            w63 => W63Provider$(;$scope_type)?,
            w64 => W64Provider$(;$scope_type)?,
            w65 => W65Provider$(;$scope_type)?,
            w66 => W66Provider$(;$scope_type)?,
            w67 => W67Provider$(;$scope_type)?,
            w68 => W68Provider$(;$scope_type)?,
            w69 => W69Provider$(;$scope_type)?,
            w70 => W70Provider$(;$scope_type)?,
            w71 => W71Provider$(;$scope_type)?,
            w72 => W72Provider$(;$scope_type)?,
            w73 => W73Provider$(;$scope_type)?,
            w74 => W74Provider$(;$scope_type)?,
            w75 => W75Provider$(;$scope_type)?,
            w76 => W76Provider$(;$scope_type)?,
            w77 => W77Provider$(;$scope_type)?,
            w78 => W78Provider$(;$scope_type)?,
            w79 => W79Provider$(;$scope_type)?,
            w80 => W80Provider$(;$scope_type)?,
            w81 => W81Provider$(;$scope_type)?,
            w82 => W82Provider$(;$scope_type)?,
            w83 => W83Provider$(;$scope_type)?,
            w84 => W84Provider$(;$scope_type)?,
            w85 => W85Provider$(;$scope_type)?,
            w86 => W86Provider$(;$scope_type)?,
            w87 => W87Provider$(;$scope_type)?,
            w88 => W88Provider$(;$scope_type)?,
            w89 => W89Provider$(;$scope_type)?,
            w90 => W90Provider$(;$scope_type)?,
            w91 => W91Provider$(;$scope_type)?,
            w92 => W92Provider$(;$scope_type)?,
            w93 => W93Provider$(;$scope_type)?,
            w94 => W94Provider$(;$scope_type)?,
            w95 => W95Provider$(;$scope_type)?,
            w96 => W96Provider$(;$scope_type)?,
            w97 => W97Provider$(;$scope_type)?,
            w98 => W98Provider$(;$scope_type)?,
            w99 => W99Provider$(;$scope_type)?,
            w100 => W100Provider$(;$scope_type)?,
        }
    }
}

pub fn wide_transient_dependencies(c: &mut Criterion) {
    let container = make_wide_container!();
    c.bench_function("wide transient", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

pub fn wide_singleton_dependencies(c: &mut Criterion) {
    let container = make_wide_container!(singleton);
    c.bench_function("wide singleton", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

pub fn wide_scoped_dependencies(c: &mut Criterion) {
    let container = make_wide_container!(scoped);
    c.bench_function("wide scoped", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

pub fn scoped_container_wide_transient_dependencies(c: &mut Criterion) {
    let container = make_wide_container!();
    let container = container.scoped();
    c.bench_function("scoped wide transient", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

pub fn scoped_container_wide_singleton_dependencies(c: &mut Criterion) {
    let container = make_wide_container!(singleton);
    let container = container.scoped();
    c.bench_function("scoped wide singleton", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

pub fn scoped_container_wide_scoped_dependencies(c: &mut Criterion) {
    let container = make_wide_container!(scoped);
    let container = container.scoped();
    c.bench_function("scoped wide scoped", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

pub fn doubly_scoped_container_wide_transient_dependencies(c: &mut Criterion) {
    let container = make_wide_container!();
    let container = container.scoped();
    let container = container.scoped();
    c.bench_function("double scoped wide transient", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

pub fn doubly_scoped_container_wide_singleton_dependencies(c: &mut Criterion) {
    let container = make_wide_container!(singleton);
    let container = container.scoped();
    let container = container.scoped();
    c.bench_function("double scoped wide singleton", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

pub fn doubly_scoped_container_wide_scoped_dependencies(c: &mut Criterion) {
    let container = make_wide_container!(scoped);
    let container = container.scoped();
    let container = container.scoped();
    c.bench_function("double scoped wide scoped", |b| {
        b.iter(|| container.resolve::<dyn IW1 + Send + Sync>("w1").unwrap())
    });
}

macro_rules! make_dep {
    (@expr $e:expr) => {
        $e
    };
    ($trayt:ident, $strukt:ident, [$($dep_name:ident => $dep_trait:ident),*]) => {
        trait $trayt {}
        #[allow(dead_code)]
        #[coi(provides dyn $trayt + Send + Sync with make_dep!(@expr $strukt::new($($dep_name),*)))]
        struct $strukt {
            $(
                #[coi(inject)]
                $dep_name: Arc<dyn $dep_trait + Send + Sync>,
            )*
        }

        impl $strukt {
            pub fn new($($dep_name: Arc<dyn $dep_trait + Send + Sync>),*) -> Self {
                Self { $($dep_name,)* }
            }
        }

        impl $trayt for $strukt {}
    }
}

make_dep!(IW1, W1, [
    w2 => IW2,
    w3 => IW3,
    w4 => IW4,
    w5 => IW5,
    w6 => IW6,
    w7 => IW7,
    w8 => IW8,
    w9 => IW9,
    w10 => IW10,
    w11 => IW11,
    w12 => IW12,
    w13 => IW13,
    w14 => IW14,
    w15 => IW15,
    w16 => IW16,
    w17 => IW17,
    w18 => IW18,
    w19 => IW19,
    w20 => IW20,
    w21 => IW21,
    w22 => IW22,
    w23 => IW23,
    w24 => IW24,
    w25 => IW25,
    w26 => IW26,
    w27 => IW27,
    w28 => IW28,
    w29 => IW29,
    w30 => IW30,
    w31 => IW31,
    w32 => IW32,
    w33 => IW33,
    w34 => IW34,
    w35 => IW35,
    w36 => IW36,
    w37 => IW37,
    w38 => IW38,
    w39 => IW39,
    w40 => IW40,
    w41 => IW41,
    w42 => IW42,
    w43 => IW43,
    w44 => IW44,
    w45 => IW45,
    w46 => IW46,
    w47 => IW47,
    w48 => IW48,
    w49 => IW49,
    w50 => IW50,
    w51 => IW51,
    w52 => IW52,
    w53 => IW53,
    w54 => IW54,
    w55 => IW55,
    w56 => IW56,
    w57 => IW57,
    w58 => IW58,
    w59 => IW59,
    w60 => IW60,
    w61 => IW61,
    w62 => IW62,
    w63 => IW63,
    w64 => IW64,
    w65 => IW65,
    w66 => IW66,
    w67 => IW67,
    w68 => IW68,
    w69 => IW69,
    w70 => IW70,
    w71 => IW71,
    w72 => IW72,
    w73 => IW73,
    w74 => IW74,
    w75 => IW75,
    w76 => IW76,
    w77 => IW77,
    w78 => IW78,
    w79 => IW79,
    w80 => IW80,
    w81 => IW81,
    w82 => IW82,
    w83 => IW83,
    w84 => IW84,
    w85 => IW85,
    w86 => IW86,
    w87 => IW87,
    w88 => IW88,
    w89 => IW89,
    w90 => IW90,
    w91 => IW91,
    w92 => IW92,
    w93 => IW93,
    w94 => IW94,
    w95 => IW95,
    w96 => IW96,
    w97 => IW97,
    w98 => IW98,
    w99 => IW99,
    w100 => IW100
]);

make_dep!(IW2, W2, []);
make_dep!(IW3, W3, []);
make_dep!(IW4, W4, []);
make_dep!(IW5, W5, []);
make_dep!(IW6, W6, []);
make_dep!(IW7, W7, []);
make_dep!(IW8, W8, []);
make_dep!(IW9, W9, []);
make_dep!(IW10, W10, []);
make_dep!(IW11, W11, []);
make_dep!(IW12, W12, []);
make_dep!(IW13, W13, []);
make_dep!(IW14, W14, []);
make_dep!(IW15, W15, []);
make_dep!(IW16, W16, []);
make_dep!(IW17, W17, []);
make_dep!(IW18, W18, []);
make_dep!(IW19, W19, []);
make_dep!(IW20, W20, []);
make_dep!(IW21, W21, []);
make_dep!(IW22, W22, []);
make_dep!(IW23, W23, []);
make_dep!(IW24, W24, []);
make_dep!(IW25, W25, []);
make_dep!(IW26, W26, []);
make_dep!(IW27, W27, []);
make_dep!(IW28, W28, []);
make_dep!(IW29, W29, []);
make_dep!(IW30, W30, []);
make_dep!(IW31, W31, []);
make_dep!(IW32, W32, []);
make_dep!(IW33, W33, []);
make_dep!(IW34, W34, []);
make_dep!(IW35, W35, []);
make_dep!(IW36, W36, []);
make_dep!(IW37, W37, []);
make_dep!(IW38, W38, []);
make_dep!(IW39, W39, []);
make_dep!(IW40, W40, []);
make_dep!(IW41, W41, []);
make_dep!(IW42, W42, []);
make_dep!(IW43, W43, []);
make_dep!(IW44, W44, []);
make_dep!(IW45, W45, []);
make_dep!(IW46, W46, []);
make_dep!(IW47, W47, []);
make_dep!(IW48, W48, []);
make_dep!(IW49, W49, []);
make_dep!(IW50, W50, []);
make_dep!(IW51, W51, []);
make_dep!(IW52, W52, []);
make_dep!(IW53, W53, []);
make_dep!(IW54, W54, []);
make_dep!(IW55, W55, []);
make_dep!(IW56, W56, []);
make_dep!(IW57, W57, []);
make_dep!(IW58, W58, []);
make_dep!(IW59, W59, []);
make_dep!(IW60, W60, []);
make_dep!(IW61, W61, []);
make_dep!(IW62, W62, []);
make_dep!(IW63, W63, []);
make_dep!(IW64, W64, []);
make_dep!(IW65, W65, []);
make_dep!(IW66, W66, []);
make_dep!(IW67, W67, []);
make_dep!(IW68, W68, []);
make_dep!(IW69, W69, []);
make_dep!(IW70, W70, []);
make_dep!(IW71, W71, []);
make_dep!(IW72, W72, []);
make_dep!(IW73, W73, []);
make_dep!(IW74, W74, []);
make_dep!(IW75, W75, []);
make_dep!(IW76, W76, []);
make_dep!(IW77, W77, []);
make_dep!(IW78, W78, []);
make_dep!(IW79, W79, []);
make_dep!(IW80, W80, []);
make_dep!(IW81, W81, []);
make_dep!(IW82, W82, []);
make_dep!(IW83, W83, []);
make_dep!(IW84, W84, []);
make_dep!(IW85, W85, []);
make_dep!(IW86, W86, []);
make_dep!(IW87, W87, []);
make_dep!(IW88, W88, []);
make_dep!(IW89, W89, []);
make_dep!(IW90, W90, []);
make_dep!(IW91, W91, []);
make_dep!(IW92, W92, []);
make_dep!(IW93, W93, []);
make_dep!(IW94, W94, []);
make_dep!(IW95, W95, []);
make_dep!(IW96, W96, []);
make_dep!(IW97, W97, []);
make_dep!(IW98, W98, []);
make_dep!(IW99, W99, []);
make_dep!(IW100, W100, []);
