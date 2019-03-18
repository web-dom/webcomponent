use webcomponent::*;
extern crate chrono;
use chrono::{DateTime, NaiveDateTime, Utc};

extern "C" {
    pub fn global_getWindow() -> Element;
    pub fn global_createEventListener() -> Element;
    pub fn global_getProperty(obj: Element, name: CString) -> Element;
    pub fn EventTarget_addEventListener(element: Element, eventName: CString, callback: Callback);
    pub fn Element_set_innerHTML(element: Element, text: CString);
    pub fn CustomElement_define(name: CString);
    pub fn console_log(message: CString);
    pub fn Window_setInterval(window: Element, callback: Callback, milliseconds: i32);
    pub fn Date_nowSeconds() -> i32;
    pub fn Date_getTimezoneOffset() -> i32;
}

struct XClock {
    element: i32,
}

impl XClock {
    fn create(element: i32) {
        unsafe {
            let x = XClock { element: element };
            x.render();
            let id = add_component(x);
            let cb = global_createEventListener();
            let window = global_getWindow();
            Window_setInterval(window, cb, 1000);
            add_callback(
                cb,
                Box::new(move |_| {
                    get_component::<XClock>(id).timer();
                }),
            );
        }
    }

    fn timer(&self) {
        self.render();
    }

    fn render(&self) {
        unsafe {
            let d = Date_nowSeconds();
            let o = Date_getTimezoneOffset();
            let now: DateTime<Utc> = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp((d - (o * 60)) as i64, 0),
                Utc,
            );
            Element_set_innerHTML(
                self.element,
                cstr(&format!("{}", now.format("%I:%M:%S %p"))),
            );
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
                XClock::create(element);
            }),
        );
        CustomElement_define(cstr("x-clock"));
    }
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: i32) {
    // this function routes callbacks to the right closure
    route_callback(callback_id, event);
}
