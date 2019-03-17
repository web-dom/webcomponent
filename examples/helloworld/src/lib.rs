use webcomponent::*;

extern "C" {
    pub fn global_getWindow() -> Element;
    pub fn global_createEventListener() -> Element;
    pub fn global_getProperty(obj: Element, name: CString) -> Element;
    pub fn EventTarget_addEventListener(element: Element, eventName: CString, callback: Callback);
    pub fn Element_set_innerHTML(element: Element, text: CString);
    pub fn CustomElement_define(name: CString,attributes: CString);
}

pub struct HelloWorld {}

impl HelloWorld {
    pub fn create(element: Element) {
        unsafe {
            Element_set_innerHTML(element, cstr("Hello World"));
        }
    }
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
                HelloWorld::create(element);
            }),
        );
        CustomElement_define(cstr("hello-world"),cstr("blah"));
    }
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: i32) {
    // this function routes callbacks to the right closure
    route_callback(callback_id, event);
}
