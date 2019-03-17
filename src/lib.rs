pub use callback::{add_callback, route_callback};
pub use cstring::cstr;
extern crate anymap;
use anymap::AnyMap;
// A global store of components to prevent deallocation
static mut COMPONENTS: Option<AnyMap> = None;
pub fn get_components<T>() -> &'static mut Vec<T> {
    unsafe {
        if COMPONENTS.is_none() {
            COMPONENTS = Some(AnyMap::new());
        }
        let components = COMPONENTS.as_mut().unwrap();
        let clocks = components.get::<Vec<T>>();
        if clocks.is_none() {
            components.insert::<Vec<T>>(Vec::new());
        }
        components.get_mut::<Vec<T>>().unwrap()
    }
}

pub fn get_component<T>(id: usize) -> &'static T {
    &get_components::<T>()[id]
}
