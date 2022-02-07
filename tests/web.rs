//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use tar_wasm::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn create_tar() {
    let files = vec![
        FileToArchive {
            name: "foo.txt".to_string(),
            content: "foo".as_bytes().to_vec(),
        },
        FileToArchive {
            name: "bar.txt".to_string(),
            content: "bar".as_bytes().to_vec(),
        },
    ];
    let tarBuilder = TarBuilder::new();
    for file in files {
        tarBuilder.add_file(&file.name, &file.content);
    }
    let tar = tarBuilder.finish();
    assert!(tar.len() > 0);
}
