use webcomponent::*;

extern "C" {
    pub fn global_getWindow() -> Element;
    pub fn global_createEventListener() -> Element;
    pub fn global_getProperty(obj: Element, name: CString) -> Element;
    pub fn EventTarget_addEventListener(element: Element, eventName: CString, callback: Callback);
    pub fn Element_set_innerHTML(element: Element, text: CString);
    pub fn CustomElement_defineWithAttributes(name: CString, attributes: CString);
    pub fn Element_attachShadow(element: Element) -> Element;
    pub fn Element_getAttribute(element: Element, attr: CString) -> CString;
    pub fn console_error(message: CString);
}

pub struct ColorText {
    element: Element,
    shadow: Element,
}

impl ColorText {
    fn create(custom_elements: &mut CustomElements, element: Element) {
        unsafe {
            let shadow = Element_attachShadow(element);
            let id = custom_elements.add(ColorText {
                element: element,
                shadow: shadow,
            });

            let mut cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("connected"), cb);
            custom_elements.add_callback(
                cb,
                Box::new(move |custom_elements,_| {
                    custom_elements.get::<ColorText>(id).connected();
                }),
            );

            cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("attributechanged"), cb);
            custom_elements.add_callback(
                cb,
                Box::new(move |custom_elements,event| {
                    custom_elements.get::<ColorText>(id).attribute_changed(event);
                }),
            );
        }
    }

    fn connected(&self) {
        self.render();
    }

    fn attribute_changed(&self, _event: i32) {
        self.render();
    }
    fn render(&self) {
        unsafe {
            let c = Element_getAttribute(self.element, cstr("color"));
            Element_set_innerHTML(
                self.shadow,
                cstr(&format!(
                    "<style>:host{{color:{} }}</style><div><slot></slot></div>",
                    cstr_to_string(c)
                )),
            );
        }
    }
}


thread_local! {
    static CUSTOM_ELEMENTS:std::cell::RefCell<CustomElements> = std::cell::RefCell::new(CustomElements::new(
    |custom_elements, tag, element| match tag {
        "color_text" => ColorText::create(custom_elements, element),
        _ => unsafe { console_error(cstr(&format!("unknown web component {}", tag))) },
    }))
}

#[no_mangle]
pub fn main() -> () {
    // This function starts listening for hello-world components
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().define("color-text");
    });
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: i32) {
    // This function routes callbacks to the right closure
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().route_callback(callback_id, event);
    });
}
