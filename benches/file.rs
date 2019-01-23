#![feature(test)]

extern crate test;

mod data;

pub use assert_fs::prelude::*;

#[bench]
fn process_empty(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::EMPTY).unwrap();

    let corrections = scorrect::Corrections::new();
    b.iter(|| scorrect::process_file(sample_path.path(), &corrections, scorrect::print_silent));

    temp.close().unwrap();
}

#[bench]
fn process_no_tokens(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::NO_TOKENS).unwrap();

    let corrections = scorrect::Corrections::new();
    b.iter(|| scorrect::process_file(sample_path.path(), &corrections, scorrect::print_silent));

    temp.close().unwrap();
}

#[bench]
fn process_single_token(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::SINGLE_TOKEN).unwrap();

    let corrections = scorrect::Corrections::new();
    b.iter(|| scorrect::process_file(sample_path.path(), &corrections, scorrect::print_silent));

    temp.close().unwrap();
}

#[bench]
fn process_sherlock(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::SHERLOCK).unwrap();

    let corrections = scorrect::Corrections::new();
    b.iter(|| scorrect::process_file(sample_path.path(), &corrections, scorrect::print_silent));

    temp.close().unwrap();
}

#[bench]
fn process_code(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::CODE).unwrap();

    let corrections = scorrect::Corrections::new();
    b.iter(|| scorrect::process_file(sample_path.path(), &corrections, scorrect::print_silent));

    temp.close().unwrap();
}

#[bench]
fn process_corpus(b: &mut test::Bencher) {
    let temp = assert_fs::TempDir::new().unwrap();
    let sample_path = temp.child("sample");
    sample_path.write_str(data::CORPUS).unwrap();

    let corrections = scorrect::Corrections::new();
    b.iter(|| scorrect::process_file(sample_path.path(), &corrections, scorrect::print_silent));

    temp.close().unwrap();
}