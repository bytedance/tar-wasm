mod utils;

// use anyhow::Error;
use tar::{Builder, Header};
use utils::set_panic_hook;
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
        set_panic_hook();
        TarBuilder {
            builder: Builder::new(Vec::new()),
        }
    }
    pub fn add_file(&mut self, name: &str, content: &[u8]) -> Result<(), JsError> {
        let mut header = Header::new_gnu();
        header.set_size(content.len() as u64);
        self.builder.append_data(&mut header, name, content)?;
        Ok(())
    }
    pub fn finish(self) -> Result<Vec<u8>, JsError> {
        let res = self.builder.into_inner()?;
        Ok(res)
    }
}
