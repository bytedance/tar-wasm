mod utils;

use tar::{Builder, Header};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct TarBuilder {
    builder: Builder<Vec<u8>>,
}

#[wasm_bindgen]
impl TarBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TarBuilder {
        TarBuilder {
            builder: Builder::new(Vec::new()),
        }
    }
    pub fn add_file(&mut self, name: &str, content: &[u8]) {
        let mut header = Header::new_gnu();
        header.set_path(&name);
        header.set_size(content.len() as u64);
        self.builder.append(&header, content);
    }
    pub fn finish(self) -> Vec<u8> {
        self.builder.into_inner().unwrap()
    }
}
