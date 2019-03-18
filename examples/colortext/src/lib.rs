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
}

pub struct ColorText {
    element: Element,
    shadow: Element,
}

impl ColorText {
    fn create(element: Element) {
        unsafe {
            let shadow = Element_attachShadow(element);
            let id = add_component(ColorText {
                element: element,
                shadow: shadow,
            });

            let mut cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("connected"), cb);
            add_callback(
                cb,
                Box::new(move |_| {
                    get_component::<ColorText>(id).connected();
                }),
            );

            cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("attributechanged"), cb);
            add_callback(
                cb,
                Box::new(move |event| {
                    get_component::<ColorText>(id).attribute_changed(event);
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
                    cstr_from_raw(c)
                )),
            );
        }
    }
}

#[no_mangle]
pub fn malloc(_len: i32) -> i32 {
    // this is a really dumb memory allocator that always says there's free memory at the position 0
    // since we only have one string coming back from the browser via HTMLInputElement_get_value
    // we don't really have a problem
    return 0;
}

#[no_mangle]
pub fn main() -> () {
    unsafe {
        let win = global_getWindow();
        let cb = global_createEventListener();
        EventTarget_addEventListener(win, cstr("customelementcreated"), cb);
        add_callback(
            cb,
            Box::new(|event| {
                let element = global_getProperty(event, cstr("detail"));
                ColorText::create(element);
            }),
        );
        CustomElement_defineWithAttributes(cstr("color-text"), cstr("color"));
    }
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: i32) {
    // this function routes callbacks to the right closure
    route_callback(callback_id, event);
}
