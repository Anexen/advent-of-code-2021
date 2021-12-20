#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

#[bench]
fn part_a(b: &mut Bencher) {
    b.iter(|| day_18::part_a(black_box(None)));
}
