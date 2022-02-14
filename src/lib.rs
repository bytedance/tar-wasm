mod utils;

// use anyhow::Error;
use tar::{Builder, Header};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use yazi::*;

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
    enable_gzip: bool,
}

#[wasm_bindgen]
impl TarBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TarBuilder {
        set_panic_hook();
        TarBuilder {
            builder: Builder::new(Vec::new()),
            enable_gzip: false,
        }
    }
    pub fn add_file(&mut self, name: &str, content: &[u8]) -> Result<(), JsError> {
        let mut header = Header::new_gnu();
        header.set_size(content.len() as u64);
        // TODO: use stricter permission
        header.set_mode(0o777);
        header.set_uid(0);
        header.set_gid(0);
        self.builder.append_data(&mut header, name, content)?;
        Ok(())
    }
    pub fn set_gzip(&mut self, enable_gzip: bool) {
        self.enable_gzip = enable_gzip;
    }
    pub fn finish(self) -> Result<Vec<u8>, JsError> {
        let tar_buffer = self.builder.into_inner()?;
        if self.enable_gzip {
            let tar_gz_buffer = compress(&tar_buffer, Format::Zlib, CompressionLevel::Default);
            match tar_gz_buffer {
                Ok(tar_gz_buffer) => Ok(tar_gz_buffer),
                Err(err) => Err(JsError::new(&format!("{:?}", err))),
            }
        } else {
            Ok(tar_buffer)
        }
    }
}
