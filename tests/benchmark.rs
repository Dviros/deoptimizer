
#![feature(test)]
extern crate test;

use test::Bencher;
use deoptimizer::{x86_64, arm, riscv};

#[bench]
fn bench_x86_64_analyze(b: &mut Bencher) {
    let binary = vec![0x90; 1024]; // Example binary data
    let mut deopt = x86_64::Deoptimizer::new();
    b.iter(|| {
        deopt.analyze(&binary).unwrap();
    });
}

#[bench]
fn bench_arm_analyze(b: &mut Bencher) {
    let binary = vec![0x00, 0x00, 0xA0, 0xE1; 256]; // Example binary data
    let mut deopt = arm::Deoptimizer::new();
    b.iter(|| {
        deopt.analyze(&binary).unwrap();
    });
}

#[bench]
fn bench_riscv_analyze(b: &mut Bencher) {
    let binary = vec![0x13, 0x00, 0x00, 0x00; 256]; // Example binary data
    let mut deopt = riscv::Deoptimizer::new();
    b.iter(|| {
        deopt.analyze(&binary).unwrap();
    });
}

#[bench]
fn bench_x86_64_transform(b: &mut Bencher) {
    let binary = vec![0x90; 1024]; // Example binary data
    let mut deopt = x86_64::Deoptimizer::new();
    deopt.analyze(&binary).unwrap();
    b.iter(|| {
        deopt.transform(&mut binary.clone()).unwrap();
    });
}

#[bench]
fn bench_arm_transform(b: &mut Bencher) {
    let binary = vec![0x00, 0x00, 0xA0, 0xE1; 256]; // Example binary data
    let mut deopt = arm::Deoptimizer::new();
    deopt.analyze(&binary).unwrap();
    b.iter(|| {
        deopt.transform(&mut binary.clone()).unwrap();
    });
}

#[bench]
fn bench_riscv_transform(b: &mut Bencher) {
    let binary = vec![0x13, 0x00, 0x00, 0x00; 256]; // Example binary data
    let mut deopt = riscv::Deoptimizer::new();
    deopt.analyze(&binary).unwrap();
    b.iter(|| {
        deopt.transform(&mut binary.clone()).unwrap();
    });
}

#[bench]
fn bench_x86_64_encode(b: &mut Bencher) {
    let binary = vec![0x90; 1024]; // Example binary data
    let mut deopt = x86_64::Deoptimizer::new();
    deopt.analyze(&binary).unwrap();
    let transformed = deopt.transform(&mut binary.clone()).unwrap();
    b.iter(|| {
        deopt.encode(&transformed).unwrap();
    });
}

#[bench]
fn bench_arm_encode(b: &mut Bencher) {
    let binary = vec![0x00, 0x00, 0xA0, 0xE1; 256]; // Example binary data
    let mut deopt = arm::Deoptimizer::new();
    deopt.analyze(&binary).unwrap();
    let transformed = deopt.transform(&mut binary.clone()).unwrap();
    b.iter(|| {
        deopt.encode(&transformed).unwrap();
    });
}

#[bench]
fn bench_riscv_encode(b: &mut Bencher) {
    let binary = vec![0x13, 0x00, 0x00, 0x00; 256]; // Example binary data
    let mut deopt = riscv::Deoptimizer::new();
    deopt.analyze(&binary).unwrap();
    let transformed = deopt.transform(&mut binary.clone()).unwrap();
    b.iter(|| {
        deopt.encode(&transformed).unwrap();
    });
}
