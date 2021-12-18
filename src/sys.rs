use web_sys;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no window")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no document")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("no have body")
}

pub fn create_element(tag: &str) -> web_sys::Element {
    document()
        .create_element(tag)
        .expect(format!("no create element {}", tag).as_str())
}
