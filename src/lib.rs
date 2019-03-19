use anymap::AnyMap;
pub use cstring::{cstr, cstr_to_string, CString};
use std::collections::HashMap;
pub type Callback = i32;
pub type Element = i32;
pub type CustomElement = usize;

extern "C" {
    pub fn global_getWindow() -> Element;
    pub fn global_createEventListener() -> Element;
    pub fn global_getProperty(obj: Element, name: CString) -> Element;
    pub fn EventTarget_addEventListener(element: Element, eventName: CString, callback: Callback);
    pub fn CustomElement_define(name: CString);
    pub fn Element_get_tagName(element: Element) -> CString;
}

pub struct CustomElements {
    pub components: AnyMap,
    pub processor: Box<fn(&mut CustomElements, &str, i32)>,
    pub callbacks: HashMap<Callback, Box<dyn FnMut(&mut CustomElements, i32)>>,
}

impl CustomElements {
    pub fn new(processor: fn(&mut CustomElements, &str, i32)) -> CustomElements {
        let mut c = CustomElements {
            components: AnyMap::new(),
            processor: Box::new(processor),
            callbacks: HashMap::new(),
        };
        unsafe {
            let win = global_getWindow();
            let cb = global_createEventListener();
            EventTarget_addEventListener(win, cstr("customelementcreated"), cb);
            let p = Box::new(move |controller: &mut CustomElements, event| {
                let element = global_getProperty(event, cstr("detail"));
                let tag = cstr_to_string(Element_get_tagName(element)).to_lowercase();
                (controller.processor)(controller, &tag, element);
            });
            c.add_callback(cb, p);
        }
        c
    }

    fn all<T: 'static>(&mut self) -> &mut Vec<T> {
        let components = self.components.get::<Vec<T>>();
        if components.is_none() {
            self.components.insert::<Vec<T>>(Vec::new());
        }
        self.components.get_mut::<Vec<T>>().unwrap()
    }

    pub fn add<T: 'static>(&mut self, item: T) -> CustomElement {
        let v = self.all();
        v.push(item);
        v.len() - 1
    }

    pub fn get<T: 'static>(&mut self, id: CustomElement) -> &T {
        &self.all::<T>()[id]
    }

    pub fn add_callback(
        &mut self,
        c: Callback,
        f: Box<dyn for<'r> FnMut(&mut CustomElements, i32)>,
    ) {
        self.callbacks.insert(c, f);
    }

    pub fn define(&self, s: &str) {
        unsafe {
            CustomElement_define(cstr(s));
        }
    }

    pub fn route_callback(&mut self, callback_id: Callback, event: i32) {
        let mut f = if let Some(k) = self.callbacks.remove(&callback_id) {
            k
        } else {
            panic!("callback does not exist");
        };
        f(self, event);
        self.add_callback(callback_id, f);
    }
}

#[no_mangle]
pub fn malloc(_len: i32) -> i32 {
    // this is a really dumb memory allocator that always says there's free memory at the position 0
    // since we only have one string coming back from the browser via HTMLInputElement_get_value
    // we don't really have a problem
    return 0;
}
