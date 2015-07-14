#![allow(non_snake_case)]

use random::{self, Source};
use std::default::Default;
use std::fs;
use std::path::PathBuf;
use test::Bencher;

use temperature::Analysis;
use temperature::model::HotSpot;

#[bench] fn step_0001(bench: &mut Bencher) { step(   1, bench); }
#[bench] fn step_0010(bench: &mut Bencher) { step(  10, bench); }
#[bench] fn step_0100(bench: &mut Bencher) { step( 100, bench); }
#[bench] fn step_1000(bench: &mut Bencher) { step(1000, bench); }

fn step(steps: usize, bench: &mut Bencher) {
    let cores = 32;

    let mut analysis = setup("032");
    let P = random::default().iter().take(steps * cores).collect::<Vec<_>>();
    let mut Q = vec![0.0; steps * cores];

    bench.iter(|| analysis.step(&P, &mut Q));
}

fn setup(name: &str) -> Analysis {
    let circuit = HotSpot::new(find(&format!("{}.flp", name)), find("hotspot.config"));
    Analysis::new(&circuit.unwrap(), &Default::default()).unwrap()
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/model/hotspot/fixtures").join(name);
    assert!(fs::metadata(&path).is_ok());
    path
}