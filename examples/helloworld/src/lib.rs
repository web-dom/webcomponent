use webcomponent::*;

pub struct HelloWorld {}
impl HelloWorld {
    pub fn create(_custom_elements: &CustomElements, element: Element) {
        element::set_inner_html(element, "Hello World!");
    }
}

thread_local! {
    static CUSTOM_ELEMENTS:std::cell::RefCell<CustomElements> = std::cell::RefCell::new(CustomElements::new(
    |custom_elements, tag, element| match tag {
        "hello-world" => HelloWorld::create(custom_elements, element),
        _ => console::error(&format!("unknown web component {}", tag)),
    }))
}

#[no_mangle]
pub fn main() -> () {
    // This function starts listening for hello-world components
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().define("hello-world");
    });
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: Event) {
    // This function routes callbacks to the right closure
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().route_callback(callback_id, event);
    });
}
