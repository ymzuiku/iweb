pub mod fetch;
pub mod sys;
pub mod theme;

use js_sys;
use serde_json;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;

pub fn perf_to_system(amt: f64) -> SystemTime {
    let secs = (amt as u64) / 1_000;
    let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
    UNIX_EPOCH + Duration::new(secs, nanos)
}

pub fn now() -> SystemTime {
    perf_to_system(js_sys::Date::now())
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ele {
    pub ele: web_sys::HtmlElement,
}

impl From<web_sys::HtmlElement> for Ele {
    fn from(e: web_sys::HtmlElement) -> Ele {
        Ele { ele: e }
    }
}

impl From<web_sys::Element> for Ele {
    fn from(e: web_sys::Element) -> Ele {
        Ele {
            ele: web_sys::HtmlElement::from(JsValue::from(e)),
        }
    }
}

impl From<web_sys::Document> for Ele {
    fn from(e: web_sys::Document) -> Ele {
        Ele {
            ele: web_sys::HtmlElement::from(JsValue::from(e)),
        }
    }
}

impl From<web_sys::Node> for Ele {
    fn from(e: web_sys::Node) -> Ele {
        Ele {
            ele: web_sys::HtmlElement::from(JsValue::from(e)),
        }
    }
}

impl From<JsValue> for Ele {
    fn from(e: JsValue) -> Ele {
        Ele {
            ele: web_sys::HtmlElement::from(e),
        }
    }
}

pub fn tag(tag: &str) -> Ele {
    Ele::from(sys::create_element(tag))
}

pub fn body() -> Ele {
    Ele::from(sys::body())
}

pub fn document() -> Ele {
    Ele {
        ele: web_sys::HtmlElement::from(JsValue::from(
            web_sys::window().unwrap().document().unwrap(),
        )),
    }
}

pub fn get_element_by_id(id: &str) -> Option<Ele> {
    if let Some(e) = sys::document().get_element_by_id(id) {
        return Some(Ele::from(e));
    }
    None
}

pub fn log(data: serde_json::Value) {
    web_sys::console::log_1(&JsValue::from_str(data.to_string().as_str()));
}

impl Ele {
    pub fn append(self, node: &Ele) -> Self {
        if let Err(e) = self.ele.append_child(&node.ele) {
            web_sys::console::log(&js_sys::Array::from(&e));
        };
        self
    }
    pub fn query_selector(self, txt: &str) -> Option<Ele> {
        if let Some(ele) = self.ele.query_selector(txt).expect("query_selecor error") {
            return Some(Ele::from(ele));
        }
        None
    }
    pub fn query_selector_all(self, txt: &str) -> Vec<Ele> {
        let list = self
            .ele
            .query_selector_all(txt)
            .expect("query_selecor error");

        let mut out = Vec::new();
        for i in 0..list.length() {
            if let Some(ele) = list.item(i) {
                out.push(Ele::from(ele));
            }
        }
        out
    }
    pub fn text(self, txt: &str) -> Self {
        self.ele.set_text_content(Some(txt));
        self
    }
    pub fn id(self, txt: &str) -> Self {
        self.ele.set_id(txt);
        self
    }
    pub fn class(self, txt: &str) -> Self {
        self.ele.set_class_name(txt);
        self
    }
    pub fn attr(self, name: &str, txt: &str) -> Self {
        self.ele
            .set_attribute(name, txt)
            .expect(format!("can not set attribute: {} {}", name, txt).as_str());

        self
    }
    pub fn inner_html(self, txt: &str) -> Self {
        self.ele.set_inner_html(txt);
        self
    }
    pub fn inner_text(self, txt: &str) -> Self {
        self.ele.set_inner_text(txt);
        self
    }
}
