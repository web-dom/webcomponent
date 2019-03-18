pub type Element = i32;
pub type Callback = i32;
pub type CString = i32;
pub use callback::{add_callback, route_callback};
use globals;
use std::ffi::CStr;

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

pub fn cstr(s: &str) -> CString {
    cstring::cstr(s)
}

pub fn cstr_from_raw(p: CString) -> String {
    let s: &CStr = unsafe { CStr::from_ptr(p as *const i8) };
    s.to_str().unwrap().to_owned()
}
