#![feature(test)]
use std::{fs, env};
extern crate test;
use test::Bencher;

use retomizer::{Config, Retomizer};

#[test]
fn test_module() {
    let pwd = env::current_dir().unwrap();
    let content = fs::read_to_string(pwd.join("example/src/pages/index.html").canonicalize().unwrap()).unwrap();
    let config = fs::read_to_string(pwd.join("example/retomizer.config.json").canonicalize().unwrap()).unwrap();

    let config: Config = serde_json::from_str(&config).unwrap();

    let mut retomizer = Retomizer::new(&config);

    retomizer.push_content(content);
    let css = retomizer.get_css();

    assert!(css != "");
}

#[bench]
fn bench_module(bench: &mut Bencher) {
    bench.iter(|| {
        let pwd = env::current_dir().unwrap();
        let content = fs::read_to_string(pwd.join("example/src/pages/index.html").canonicalize().unwrap()).unwrap();
        let config = fs::read_to_string(pwd.join("example/retomizer.config.json").canonicalize().unwrap()).unwrap();

        let config: Config = serde_json::from_str(&config).unwrap();

        let mut retomizer = Retomizer::new(&config);

        retomizer.push_content(content);
        return retomizer.get_css();
    })
}
