#![feature(test)]
use std::fs;
extern crate test;
use test::Bencher;

use retomizer::{Config, Retomizer};

#[test]
fn test_module() {
    let content = fs::read_to_string("./tests/files/index.html").unwrap();
    let config = fs::read_to_string("./tests/files/retomizer.config.json").unwrap();

    let config: Config = serde_json::from_str(&config).unwrap();

    let mut retomizer = Retomizer::new(&config);

    retomizer.push_content(content);
    let css = retomizer.get_css();

    // println!("{css}");
    // assert!(false);
}

#[bench]
fn bench_module(bench: &mut Bencher) {
    bench.iter(|| {
        let content = fs::read_to_string("./tests/files/index.html").unwrap();
        let config = fs::read_to_string("./tests/files/retomizer.config.json").unwrap();

        let config: Config = serde_json::from_str(&config).unwrap();

        let mut retomizer = Retomizer::new(&config);

        retomizer.push_content(content);
        return retomizer.get_css();
    })
}
