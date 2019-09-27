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
    router.register("/02-webgl/triangle", Box::new(crate::pages::webgl_02_triangle::run));
    router.register("/03-webgl/triangle_lines", Box::new(crate::pages::webgl_03_triangle_lines::run));
    router.register("/04-webgl/triangle_points", Box::new(crate::pages::webgl_04_triangle_points::run));
    router.register("/05-webgl/cube", Box::new(crate::pages::webgl_05_cube::run));
    router.register("/06-webgl/camera", Box::new(crate::pages::webgl_06_camera::run));
    router.register("/07-webgl/light", Box::new(crate::pages::webgl_07_light::run));
    router.route().expect("some route error");
    Ok(())
}
