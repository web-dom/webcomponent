pub type Element = i32;
pub type Callback = i32;
pub use callback::{add_callback, route_callback};
pub use cstring::{CString,cstr,cstr_to_string};
use globals;

pub fn get_components<T>() -> &'static mut Vec<T> {
    globals::get_all::<T>()
}

pub fn get_component<T>(id: usize) -> &'static T {
    &get_components::<T>()[id]
}

pub fn add_component<T>(item: T) -> globals::Global
where
    T: 'static,
{
    globals::add::<T>(item)
}
