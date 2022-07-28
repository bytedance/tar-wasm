// Copyright (c) 2022 Bytedance Inc.
// 
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
// 
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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
