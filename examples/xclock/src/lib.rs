use webcomponent::*;
extern crate chrono;
use chrono::{DateTime, NaiveDateTime, Utc};

extern "C" {
    pub fn global_getWindow() -> Element;
    pub fn global_createEventListener() -> Element;
    pub fn EventTarget_addEventListener(element: Element, eventName: CString, callback: Callback);
    pub fn Element_set_innerHTML(element: Element, text: CString);
    pub fn console_error(message: CString);
    pub fn Window_setInterval(window: Element, callback: Callback, milliseconds: i32);
    pub fn Date_nowSeconds() -> i32;
    pub fn Date_getTimezoneOffset() -> i32;
}

struct XClock {
    element: i32,
}

impl XClock {
    fn create(custom_elements: &mut CustomElements, element: i32) {
        unsafe {
            let x = XClock { element: element };
            x.render();
            let id = custom_elements.add(x);
            let cb = global_createEventListener();
            let window = global_getWindow();
            Window_setInterval(window, cb, 1000);
            custom_elements.add_callback(
                cb,
                Box::new(move |custom_elements, _event| {
                    custom_elements.get::<XClock>(id).timer();
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

thread_local! {
    static CUSTOM_ELEMENTS:std::cell::RefCell<CustomElements> = std::cell::RefCell::new(CustomElements::new(
    |custom_elements, tag, element| match tag {
        "x-clock" => XClock::create(custom_elements, element),
        _ => unsafe { console_error(cstr(&format!("unknown web component {}", tag))) },
    }))
}

#[no_mangle]
pub fn main() -> () {
    // This function starts listening for hello-world components
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().define("x-clock");
    });
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: i32) {
    // This function routes callbacks to the right closure
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().route_callback(callback_id, event);
    });
}
