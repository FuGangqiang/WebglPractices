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
    router.register("/00-hello_world", Box::new(crate::pages::hello_world::run));
    router.register("/01-webgl/hello", Box::new(crate::pages::webgl_01_hello::run));
    router.register("/02-webgl/hello_app", Box::new(crate::pages::webgl_02_hello_app::run));
    router.register("/03-webgl/triangle", Box::new(crate::pages::webgl_03_triangle::run));
    router.register("/04-webgl/triangle_shader", Box::new(crate::pages::webgl_04_triangle_shader::run));
    router.route().expect("some route error");
    Ok(())
}
