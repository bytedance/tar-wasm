mod utils;

use serde::{Deserialize, Serialize};
use tar::{Builder, Header};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct FileToArchive {
    pub name: String,
    pub content: Vec<u8>,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tar-wasm!");
}

#[wasm_bindgen]
pub fn tar(files_ser: &JsValue) -> Vec<u8> {
    let files: Vec<FileToArchive> = files_ser.into_serde().unwrap();
    let mut builder = Builder::new(Vec::new());
    for file in files {
        let mut header = Header::new_gnu();
        header.set_size(file.content.len() as u64);
        header.set_path(&file.name).unwrap();
        builder.append(&header, file.content.as_slice()).unwrap();
    }
    builder.into_inner().unwrap()
}
