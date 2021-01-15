//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate simple_primitives;
use simple_primitives::shapes::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test() {
    generate_geometry(0, vec![1.5, 1.5, 1.5], vec![0.10]);
    generate_geometry(1, vec![1.5, 1.5, 1.5], vec![0.1, 0.10]);
    generate_geometry(2, vec![1.5, 1.5, 1.5], vec![0.10]);
    generate_geometry(3, vec![1.5, 1.5, 1.5], vec![0.10]);
    generate_geometry(4, vec![1.5, 1.5, 1.5], vec![0.1, 0.10]);
    generate_geometry(5, vec![1.5, 1.5, 1.5], vec![0.10, 0.8, 1.0]);
}