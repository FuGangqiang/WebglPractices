use log::debug;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

type Endpoint = Box<dyn Fn() -> Result<(), JsValue>>;

pub(crate) struct Router<'a> {
    endpoint_map: HashMap<&'a str, Endpoint>,
}

impl<'a> Router<'a> {
    pub(crate) fn new() -> Self {
        let endpoint_map = HashMap::new();
        Self { endpoint_map }
    }

    pub(crate) fn register(&mut self, path: &'a str, endpoint: Endpoint) {
        self.endpoint_map.insert(path, endpoint);
    }

    pub(crate) fn route(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();
        let mut hash_str = location.hash().expect("get hash string error");
        if hash_str.is_empty() {
            hash_str.push_str("#/");
        }
        let route_path = hash_str.trim_start_matches('#');
        debug!("current_path is: {}", route_path);
        let endpoint = self.endpoint_map.get(route_path);
        if let Some(handler) = endpoint {
            handler().unwrap();
        } else {
            self.index().unwrap();
        }
        Ok(())
    }

    fn index(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let val = document.create_element("p")?;
        val.set_inner_html("Webgl2 Practices");
        body.append_child(&val)?;

        let ul = document.create_element("ul")?;

        let mut paths: Vec<_> = self.endpoint_map.keys().collect();
        paths.sort();
        for path in paths {
            let text = path.replace('/', " ");
            let link_text = document.create_text_node(text.trim());
            let link = document.create_element("a")?;
            link.append_child(&link_text)?;
            let link_path = format!("/#{}", path);
            link.set_attribute("href", &link_path)?;
            link.set_attribute("target", "_blank")?;

            let li = document.create_element("li")?;
            li.append_child(&link)?;
            ul.append_child(&li)?;
        }
        body.append_child(&ul)?;

        Ok(())
    }
}
