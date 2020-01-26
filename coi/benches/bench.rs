#![allow(dead_code)]
#![allow(soft_unstable)]
#![feature(test)]
extern crate test;

use coi::{container, Container, Inject};
use std::sync::Arc;
use test::Bencher;

#[bench]
fn deeply_nested_dependencies(b: &mut Bencher) {
    let mut container = make_deep_container();
    b.iter(|| container.resolve::<dyn ID1>("d1"));
}

// #[bench]
// fn wide_dependencies(b: &mut Bencher) {

// }

// #[bench]
// fn deeply_scoped(b: &mut Bencher) {
//     let mut container = container! {

//     };
//     for _ in 0..1000 {
//         container = container.scoped();
//     }

//     b.iter(|| container.resolve::<dyn Bottom>("bottom"));
// }

// #[bench]
// fn singleton(b: &mut Bencher) {

// }

fn make_deep_container() -> Container {
    container! {
        d1 => D1Provider,
        d2 => D2Provider,
        d3 => D3Provider,
        d4 => D4Provider,
        d5 => D5Provider,
        d6 => D6Provider,
        d7 => D7Provider,
        d8 => D8Provider,
        d9 => D9Provider,
        d10 => D10Provider,
        d11 => D11Provider,
        d12 => D12Provider,
        d13 => D13Provider,
        d14 => D14Provider,
        d15 => D15Provider,
        d16 => D16Provider,
        d17 => D17Provider,
        d18 => D18Provider,
        d19 => D19Provider,
        d20 => D20Provider,
        d21 => D21Provider,
        d22 => D22Provider,
        d23 => D23Provider,
        d24 => D24Provider,
        d25 => D25Provider,
        d26 => D26Provider,
        d27 => D27Provider,
        d28 => D28Provider,
        d29 => D29Provider,
        d30 => D30Provider,
        d31 => D31Provider,
        d32 => D32Provider,
        d33 => D33Provider,
        d34 => D34Provider,
        d35 => D35Provider,
        d36 => D36Provider,
        d37 => D37Provider,
        d38 => D38Provider,
        d39 => D39Provider,
        d40 => D40Provider,
        d41 => D41Provider,
        d42 => D42Provider,
        d43 => D43Provider,
        d44 => D44Provider,
        d45 => D45Provider,
        d46 => D46Provider,
        d47 => D47Provider,
        d48 => D48Provider,
        d49 => D49Provider,
        d50 => D50Provider,
        d51 => D51Provider,
        d52 => D52Provider,
        d53 => D53Provider,
        d54 => D54Provider,
        d55 => D55Provider,
        d56 => D56Provider,
        d57 => D57Provider,
        d58 => D58Provider,
        d59 => D59Provider,
        d60 => D60Provider,
        d61 => D61Provider,
        d62 => D62Provider,
        d63 => D63Provider,
        d64 => D64Provider,
        d65 => D65Provider,
        d66 => D66Provider,
        d67 => D67Provider,
        d68 => D68Provider,
        d69 => D69Provider,
        d70 => D70Provider,
        d71 => D71Provider,
        d72 => D72Provider,
        d73 => D73Provider,
        d74 => D74Provider,
        d75 => D75Provider,
        d76 => D76Provider,
        d77 => D77Provider,
        d78 => D78Provider,
        d79 => D79Provider,
        d80 => D80Provider,
        d81 => D81Provider,
        d82 => D82Provider,
        d83 => D83Provider,
        d84 => D84Provider,
        d85 => D85Provider,
        d86 => D86Provider,
        d87 => D87Provider,
        d88 => D88Provider,
        d89 => D89Provider,
        d90 => D90Provider,
        d91 => D91Provider,
        d92 => D92Provider,
        d93 => D93Provider,
        d94 => D94Provider,
        d95 => D95Provider,
        d96 => D96Provider,
        d97 => D97Provider,
        d98 => D98Provider,
        d99 => D99Provider,
        d100 => D100Provider,
    }
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
make_dep!(ID19, D19, [d20 => ID10]);
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