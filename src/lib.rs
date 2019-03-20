use anymap::AnyMap;
use std::collections::HashMap;
pub use web_dom::*;
pub type Callback = i32;
pub type Element = i32;
pub type CustomElement = usize;

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
        let win = window();
        let cb = create_event_listener();
        eventtarget::add_event_listener(win, "customelementcreated", cb);
        let p = Box::new(move |controller: &mut CustomElements, event| {
            let el = get_property(event, "detail");
            let tag = element::get_tag_name(el).to_lowercase();
            (controller.processor)(controller, &tag, el);
        });
        c.add_callback(cb, p);
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
        customelement::define(s);
    }

    pub fn define_with_attributes(&self, s: &str, a: &str) {
        customelement::define_with_attributes(s, a);
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
