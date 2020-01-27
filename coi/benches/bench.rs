#![allow(dead_code)]
#![allow(soft_unstable)]
#![feature(test)]
extern crate test;

use coi::{container, Inject};
use std::sync::Arc;
use test::Bencher;

macro_rules! make_deep_container {
    ($($scope_type:ident)?) => {
        container! {
            d1 => D1Provider$(.$scope_type)?,
            d2 => D2Provider$(.$scope_type)?,
            d3 => D3Provider$(.$scope_type)?,
            d4 => D4Provider$(.$scope_type)?,
            d5 => D5Provider$(.$scope_type)?,
            d6 => D6Provider$(.$scope_type)?,
            d7 => D7Provider$(.$scope_type)?,
            d8 => D8Provider$(.$scope_type)?,
            d9 => D9Provider$(.$scope_type)?,
            d10 => D10Provider$(.$scope_type)?,
            d11 => D11Provider$(.$scope_type)?,
            d12 => D12Provider$(.$scope_type)?,
            d13 => D13Provider$(.$scope_type)?,
            d14 => D14Provider$(.$scope_type)?,
            d15 => D15Provider$(.$scope_type)?,
            d16 => D16Provider$(.$scope_type)?,
            d17 => D17Provider$(.$scope_type)?,
            d18 => D18Provider$(.$scope_type)?,
            d19 => D19Provider$(.$scope_type)?,
            d20 => D20Provider$(.$scope_type)?,
            d21 => D21Provider$(.$scope_type)?,
            d22 => D22Provider$(.$scope_type)?,
            d23 => D23Provider$(.$scope_type)?,
            d24 => D24Provider$(.$scope_type)?,
            d25 => D25Provider$(.$scope_type)?,
            d26 => D26Provider$(.$scope_type)?,
            d27 => D27Provider$(.$scope_type)?,
            d28 => D28Provider$(.$scope_type)?,
            d29 => D29Provider$(.$scope_type)?,
            d30 => D30Provider$(.$scope_type)?,
            d31 => D31Provider$(.$scope_type)?,
            d32 => D32Provider$(.$scope_type)?,
            d33 => D33Provider$(.$scope_type)?,
            d34 => D34Provider$(.$scope_type)?,
            d35 => D35Provider$(.$scope_type)?,
            d36 => D36Provider$(.$scope_type)?,
            d37 => D37Provider$(.$scope_type)?,
            d38 => D38Provider$(.$scope_type)?,
            d39 => D39Provider$(.$scope_type)?,
            d40 => D40Provider$(.$scope_type)?,
            d41 => D41Provider$(.$scope_type)?,
            d42 => D42Provider$(.$scope_type)?,
            d43 => D43Provider$(.$scope_type)?,
            d44 => D44Provider$(.$scope_type)?,
            d45 => D45Provider$(.$scope_type)?,
            d46 => D46Provider$(.$scope_type)?,
            d47 => D47Provider$(.$scope_type)?,
            d48 => D48Provider$(.$scope_type)?,
            d49 => D49Provider$(.$scope_type)?,
            d50 => D50Provider$(.$scope_type)?,
            d51 => D51Provider$(.$scope_type)?,
            d52 => D52Provider$(.$scope_type)?,
            d53 => D53Provider$(.$scope_type)?,
            d54 => D54Provider$(.$scope_type)?,
            d55 => D55Provider$(.$scope_type)?,
            d56 => D56Provider$(.$scope_type)?,
            d57 => D57Provider$(.$scope_type)?,
            d58 => D58Provider$(.$scope_type)?,
            d59 => D59Provider$(.$scope_type)?,
            d60 => D60Provider$(.$scope_type)?,
            d61 => D61Provider$(.$scope_type)?,
            d62 => D62Provider$(.$scope_type)?,
            d63 => D63Provider$(.$scope_type)?,
            d64 => D64Provider$(.$scope_type)?,
            d65 => D65Provider$(.$scope_type)?,
            d66 => D66Provider$(.$scope_type)?,
            d67 => D67Provider$(.$scope_type)?,
            d68 => D68Provider$(.$scope_type)?,
            d69 => D69Provider$(.$scope_type)?,
            d70 => D70Provider$(.$scope_type)?,
            d71 => D71Provider$(.$scope_type)?,
            d72 => D72Provider$(.$scope_type)?,
            d73 => D73Provider$(.$scope_type)?,
            d74 => D74Provider$(.$scope_type)?,
            d75 => D75Provider$(.$scope_type)?,
            d76 => D76Provider$(.$scope_type)?,
            d77 => D77Provider$(.$scope_type)?,
            d78 => D78Provider$(.$scope_type)?,
            d79 => D79Provider$(.$scope_type)?,
            d80 => D80Provider$(.$scope_type)?,
            d81 => D81Provider$(.$scope_type)?,
            d82 => D82Provider$(.$scope_type)?,
            d83 => D83Provider$(.$scope_type)?,
            d84 => D84Provider$(.$scope_type)?,
            d85 => D85Provider$(.$scope_type)?,
            d86 => D86Provider$(.$scope_type)?,
            d87 => D87Provider$(.$scope_type)?,
            d88 => D88Provider$(.$scope_type)?,
            d89 => D89Provider$(.$scope_type)?,
            d90 => D90Provider$(.$scope_type)?,
            d91 => D91Provider$(.$scope_type)?,
            d92 => D92Provider$(.$scope_type)?,
            d93 => D93Provider$(.$scope_type)?,
            d94 => D94Provider$(.$scope_type)?,
            d95 => D95Provider$(.$scope_type)?,
            d96 => D96Provider$(.$scope_type)?,
            d97 => D97Provider$(.$scope_type)?,
            d98 => D98Provider$(.$scope_type)?,
            d99 => D99Provider$(.$scope_type)?,
            d100 => D100Provider$(.$scope_type)?,
        }
    }
}

macro_rules! make_wide_container {
    ($($scope_type:ident)?) => {
        container! {
            w1 => W1Provider$(.$scope_type)?,
            w2 => W2Provider$(.$scope_type)?,
            w3 => W3Provider$(.$scope_type)?,
            w4 => W4Provider$(.$scope_type)?,
            w5 => W5Provider$(.$scope_type)?,
            w6 => W6Provider$(.$scope_type)?,
            w7 => W7Provider$(.$scope_type)?,
            w8 => W8Provider$(.$scope_type)?,
            w9 => W9Provider$(.$scope_type)?,
            w10 => W10Provider$(.$scope_type)?,
            w11 => W11Provider$(.$scope_type)?,
            w12 => W12Provider$(.$scope_type)?,
            w13 => W13Provider$(.$scope_type)?,
            w14 => W14Provider$(.$scope_type)?,
            w15 => W15Provider$(.$scope_type)?,
            w16 => W16Provider$(.$scope_type)?,
            w17 => W17Provider$(.$scope_type)?,
            w18 => W18Provider$(.$scope_type)?,
            w19 => W19Provider$(.$scope_type)?,
            w20 => W20Provider$(.$scope_type)?,
            w21 => W21Provider$(.$scope_type)?,
            w22 => W22Provider$(.$scope_type)?,
            w23 => W23Provider$(.$scope_type)?,
            w24 => W24Provider$(.$scope_type)?,
            w25 => W25Provider$(.$scope_type)?,
            w26 => W26Provider$(.$scope_type)?,
            w27 => W27Provider$(.$scope_type)?,
            w28 => W28Provider$(.$scope_type)?,
            w29 => W29Provider$(.$scope_type)?,
            w30 => W30Provider$(.$scope_type)?,
            w31 => W31Provider$(.$scope_type)?,
            w32 => W32Provider$(.$scope_type)?,
            w33 => W33Provider$(.$scope_type)?,
            w34 => W34Provider$(.$scope_type)?,
            w35 => W35Provider$(.$scope_type)?,
            w36 => W36Provider$(.$scope_type)?,
            w37 => W37Provider$(.$scope_type)?,
            w38 => W38Provider$(.$scope_type)?,
            w39 => W39Provider$(.$scope_type)?,
            w40 => W40Provider$(.$scope_type)?,
            w41 => W41Provider$(.$scope_type)?,
            w42 => W42Provider$(.$scope_type)?,
            w43 => W43Provider$(.$scope_type)?,
            w44 => W44Provider$(.$scope_type)?,
            w45 => W45Provider$(.$scope_type)?,
            w46 => W46Provider$(.$scope_type)?,
            w47 => W47Provider$(.$scope_type)?,
            w48 => W48Provider$(.$scope_type)?,
            w49 => W49Provider$(.$scope_type)?,
            w50 => W50Provider$(.$scope_type)?,
            w51 => W51Provider$(.$scope_type)?,
            w52 => W52Provider$(.$scope_type)?,
            w53 => W53Provider$(.$scope_type)?,
            w54 => W54Provider$(.$scope_type)?,
            w55 => W55Provider$(.$scope_type)?,
            w56 => W56Provider$(.$scope_type)?,
            w57 => W57Provider$(.$scope_type)?,
            w58 => W58Provider$(.$scope_type)?,
            w59 => W59Provider$(.$scope_type)?,
            w60 => W60Provider$(.$scope_type)?,
            w61 => W61Provider$(.$scope_type)?,
            w62 => W62Provider$(.$scope_type)?,
            w63 => W63Provider$(.$scope_type)?,
            w64 => W64Provider$(.$scope_type)?,
            w65 => W65Provider$(.$scope_type)?,
            w66 => W66Provider$(.$scope_type)?,
            w67 => W67Provider$(.$scope_type)?,
            w68 => W68Provider$(.$scope_type)?,
            w69 => W69Provider$(.$scope_type)?,
            w70 => W70Provider$(.$scope_type)?,
            w71 => W71Provider$(.$scope_type)?,
            w72 => W72Provider$(.$scope_type)?,
            w73 => W73Provider$(.$scope_type)?,
            w74 => W74Provider$(.$scope_type)?,
            w75 => W75Provider$(.$scope_type)?,
            w76 => W76Provider$(.$scope_type)?,
            w77 => W77Provider$(.$scope_type)?,
            w78 => W78Provider$(.$scope_type)?,
            w79 => W79Provider$(.$scope_type)?,
            w80 => W80Provider$(.$scope_type)?,
            w81 => W81Provider$(.$scope_type)?,
            w82 => W82Provider$(.$scope_type)?,
            w83 => W83Provider$(.$scope_type)?,
            w84 => W84Provider$(.$scope_type)?,
            w85 => W85Provider$(.$scope_type)?,
            w86 => W86Provider$(.$scope_type)?,
            w87 => W87Provider$(.$scope_type)?,
            w88 => W88Provider$(.$scope_type)?,
            w89 => W89Provider$(.$scope_type)?,
            w90 => W90Provider$(.$scope_type)?,
            w91 => W91Provider$(.$scope_type)?,
            w92 => W92Provider$(.$scope_type)?,
            w93 => W93Provider$(.$scope_type)?,
            w94 => W94Provider$(.$scope_type)?,
            w95 => W95Provider$(.$scope_type)?,
            w96 => W96Provider$(.$scope_type)?,
            w97 => W97Provider$(.$scope_type)?,
            w98 => W98Provider$(.$scope_type)?,
            w99 => W99Provider$(.$scope_type)?,
            w100 => W100Provider$(.$scope_type)?,
        }
    }
}

#[bench]
fn deeply_nested_transient_dependencies(b: &mut Bencher) {
    let container = make_deep_container!();
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn deeply_nested_singleton_dependencies(b: &mut Bencher) {
    let container = make_deep_container!(singleton);
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn deeply_nested_scoped_dependencies(b: &mut Bencher) {
    let container = make_deep_container!(scoped);
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn wide_transient_dependencies(b: &mut Bencher) {
    let container = make_wide_container!();
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

#[bench]
fn wide_singleton_dependencies(b: &mut Bencher) {
    let container = make_wide_container!(singleton);
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

#[bench]
fn wide_scoped_dependencies(b: &mut Bencher) {
    let container = make_wide_container!(scoped);
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

#[bench]
fn scoped_container_deeply_nested_transient_dependencies(b: &mut Bencher) {
    let container = make_deep_container!();
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn scoped_container_deeply_nested_singleton_dependencies(b: &mut Bencher) {
    let container = make_deep_container!(singleton);
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn scoped_container_deeply_nested_scoped_dependencies(b: &mut Bencher) {
    let container = make_deep_container!(scoped);
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn scoped_container_wide_transient_dependencies(b: &mut Bencher) {
    let container = make_wide_container!();
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

#[bench]
fn scoped_container_wide_singleton_dependencies(b: &mut Bencher) {
    let container = make_wide_container!(singleton);
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

#[bench]
fn scoped_container_wide_scoped_dependencies(b: &mut Bencher) {
    let container = make_wide_container!(scoped);
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

#[bench]
fn doubly_scoped_container_deeply_nested_transient_dependencies(b: &mut Bencher) {
    let container = make_deep_container!();
    let container = container.scoped();
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn doubly_scoped_container_deeply_nested_singleton_dependencies(b: &mut Bencher) {
    let container = make_deep_container!(singleton);
    let container = container.scoped();
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn doubly_scoped_container_deeply_nested_scoped_dependencies(b: &mut Bencher) {
    let container = make_deep_container!(scoped);
    let container = container.scoped();
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn ID1>("d1").unwrap());
}

#[bench]
fn doubly_scoped_container_wide_transient_dependencies(b: &mut Bencher) {
    let container = make_wide_container!();
    let container = container.scoped();
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

#[bench]
fn doubly_scoped_container_wide_singleton_dependencies(b: &mut Bencher) {
    let container = make_wide_container!(singleton);
    let container = container.scoped();
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

#[bench]
fn doubly_scoped_container_wide_scoped_dependencies(b: &mut Bencher) {
    let container = make_wide_container!(scoped);
    let container = container.scoped();
    let container = container.scoped();
    b.iter(|| container.resolve::<dyn IW1>("w1").unwrap());
}

macro_rules! make_dep {
    ($trait:ident, $struct:ident, [$($dep_name:ident => $dep_trait:ident),*]) => {
        trait $trait: Inject {}
        #[derive(Inject)]
        #[provides(dyn $trait with $struct::new($($dep_name),*))]
        struct $struct {
            $(
                #[inject]
                $dep_name: Arc<dyn $dep_trait>,
            )*
        }

        impl $struct {
            fn new($($dep_name: Arc<dyn $dep_trait>),*) -> Self {
                Self { $($dep_name,)* }
            }
        }

        impl $trait for $struct {}
    }
}
make_dep!(ID1, D1, [d2 => ID2]);
make_dep!(ID2, D2, [d3 => ID3]);
make_dep!(ID3, D3, [d4 => ID4]);
make_dep!(ID4, D4, [d5 => ID5]);
make_dep!(ID5, D5, [d6 => ID6]);
make_dep!(ID6, D6, [d7 => ID7]);
make_dep!(ID7, D7, [d8 => ID8]);
make_dep!(ID8, D8, [d9 => ID9]);
make_dep!(ID9, D9, [d10 => ID10]);
make_dep!(ID10, D10, [d11 => ID11]);
make_dep!(ID11, D11, [d12 => ID12]);
make_dep!(ID12, D12, [d13 => ID13]);
make_dep!(ID13, D13, [d14 => ID14]);
make_dep!(ID14, D14, [d15 => ID15]);
make_dep!(ID15, D15, [d16 => ID16]);
make_dep!(ID16, D16, [d17 => ID17]);
make_dep!(ID17, D17, [d18 => ID18]);
make_dep!(ID18, D18, [d19 => ID19]);
make_dep!(ID19, D19, [d20 => ID20]);
make_dep!(ID20, D20, [d21 => ID21]);
make_dep!(ID21, D21, [d22 => ID22]);
make_dep!(ID22, D22, [d23 => ID23]);
make_dep!(ID23, D23, [d24 => ID24]);
make_dep!(ID24, D24, [d25 => ID25]);
make_dep!(ID25, D25, [d26 => ID26]);
make_dep!(ID26, D26, [d27 => ID27]);
make_dep!(ID27, D27, [d28 => ID28]);
make_dep!(ID28, D28, [d29 => ID29]);
make_dep!(ID29, D29, [d30 => ID30]);
make_dep!(ID30, D30, [d31 => ID31]);
make_dep!(ID31, D31, [d32 => ID32]);
make_dep!(ID32, D32, [d33 => ID33]);
make_dep!(ID33, D33, [d34 => ID34]);
make_dep!(ID34, D34, [d35 => ID35]);
make_dep!(ID35, D35, [d36 => ID36]);
make_dep!(ID36, D36, [d37 => ID37]);
make_dep!(ID37, D37, [d38 => ID38]);
make_dep!(ID38, D38, [d39 => ID39]);
make_dep!(ID39, D39, [d40 => ID40]);
make_dep!(ID40, D40, [d41 => ID41]);
make_dep!(ID41, D41, [d42 => ID42]);
make_dep!(ID42, D42, [d43 => ID43]);
make_dep!(ID43, D43, [d44 => ID44]);
make_dep!(ID44, D44, [d45 => ID45]);
make_dep!(ID45, D45, [d46 => ID46]);
make_dep!(ID46, D46, [d47 => ID47]);
make_dep!(ID47, D47, [d48 => ID48]);
make_dep!(ID48, D48, [d49 => ID49]);
make_dep!(ID49, D49, [d50 => ID50]);
make_dep!(ID50, D50, [d51 => ID51]);
make_dep!(ID51, D51, [d52 => ID52]);
make_dep!(ID52, D52, [d53 => ID53]);
make_dep!(ID53, D53, [d54 => ID54]);
make_dep!(ID54, D54, [d55 => ID55]);
make_dep!(ID55, D55, [d56 => ID56]);
make_dep!(ID56, D56, [d57 => ID57]);
make_dep!(ID57, D57, [d58 => ID58]);
make_dep!(ID58, D58, [d59 => ID59]);
make_dep!(ID59, D59, [d60 => ID60]);
make_dep!(ID60, D60, [d61 => ID61]);
make_dep!(ID61, D61, [d62 => ID62]);
make_dep!(ID62, D62, [d63 => ID63]);
make_dep!(ID63, D63, [d64 => ID64]);
make_dep!(ID64, D64, [d65 => ID65]);
make_dep!(ID65, D65, [d66 => ID66]);
make_dep!(ID66, D66, [d67 => ID67]);
make_dep!(ID67, D67, [d68 => ID68]);
make_dep!(ID68, D68, [d69 => ID69]);
make_dep!(ID69, D69, [d70 => ID70]);
make_dep!(ID70, D70, [d71 => ID71]);
make_dep!(ID71, D71, [d72 => ID72]);
make_dep!(ID72, D72, [d73 => ID73]);
make_dep!(ID73, D73, [d74 => ID74]);
make_dep!(ID74, D74, [d75 => ID75]);
make_dep!(ID75, D75, [d76 => ID76]);
make_dep!(ID76, D76, [d77 => ID77]);
make_dep!(ID77, D77, [d78 => ID78]);
make_dep!(ID78, D78, [d79 => ID79]);
make_dep!(ID79, D79, [d80 => ID80]);
make_dep!(ID80, D80, [d81 => ID81]);
make_dep!(ID81, D81, [d82 => ID82]);
make_dep!(ID82, D82, [d83 => ID83]);
make_dep!(ID83, D83, [d84 => ID84]);
make_dep!(ID84, D84, [d85 => ID85]);
make_dep!(ID85, D85, [d86 => ID86]);
make_dep!(ID86, D86, [d87 => ID87]);
make_dep!(ID87, D87, [d88 => ID88]);
make_dep!(ID88, D88, [d89 => ID89]);
make_dep!(ID89, D89, [d90 => ID90]);
make_dep!(ID90, D90, [d91 => ID91]);
make_dep!(ID91, D91, [d92 => ID92]);
make_dep!(ID92, D92, [d93 => ID93]);
make_dep!(ID93, D93, [d94 => ID94]);
make_dep!(ID94, D94, [d95 => ID95]);
make_dep!(ID95, D95, [d96 => ID96]);
make_dep!(ID96, D96, [d97 => ID97]);
make_dep!(ID97, D97, [d98 => ID98]);
make_dep!(ID98, D98, [d99 => ID99]);
make_dep!(ID99, D99, [d100 => ID100]);
make_dep!(ID100, D100, []);

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
