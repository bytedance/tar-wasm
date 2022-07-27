//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use tar_wasm::*;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

struct FileToArchive {
    name: String,
    content: Vec<u8>,
}

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
    let mut tar_builder = TarBuilder::new();
    for file in files {
        tar_builder.add_file(&file.name, &file.content);
    }
    let tar_result = tar_builder.finish();

    match tar_result {
        Ok(tar) => {
            assert!(tar.len() > 0);
        }
        Err(_err) => {
            assert!(false);
        }
    }
}
