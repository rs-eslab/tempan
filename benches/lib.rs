#![feature(test)]

extern crate random;
extern crate temperature;
extern crate test;

use random::Source;

#[path="../tests/support/mod.rs"]
mod support;

#[bench]
#[allow(non_snake_case)]
fn compute_transient(bench: &mut test::Bencher) {
    use std::iter::repeat;

    let analysis = support::setup("032");

    let nc = 32;
    let ns = 1000;
    let nn = 4 * nc + 12;

    let P = random::default().iter().take(nc * ns).collect::<Vec<_>>();
    let mut Q = repeat(0.0).take(nc * ns).collect::<Vec<_>>();
    let mut S = repeat(0.0).take(nn * ns).collect::<Vec<_>>();

    bench.iter(|| analysis.compute_transient(&P, &mut Q, &mut S, ns));
}
