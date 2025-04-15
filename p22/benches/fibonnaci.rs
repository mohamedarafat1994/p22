#![feature(test)]

extern crate test;
use p22::calculator::fibonacci_loop;
use p22::calculator::fibonacci_rec;
use test::Bencher;

#[bench]
fn bench_fibonacci_loop(bencher: &mut Bencher) {
    bencher.iter(|| fibonacci_loop(300));
}

#[bench]
fn bench_fibonacci_rec(bencher: &mut Bencher) {
    bencher.iter(|| fibonacci_rec(300));
}
