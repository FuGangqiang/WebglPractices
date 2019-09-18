mod pages;
mod router;

use router::Router;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).expect("set log level error");
    let mut router = Router::new();
    router.register("/00-hello_world", Box::new(pages::hello_world));
    router.route().expect("some route error");
    Ok(())
}
