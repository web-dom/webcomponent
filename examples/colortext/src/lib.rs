use webcomponent::*;

pub struct ColorText {
    element: Element,
    shadow: Element,
}

impl ColorText {
    fn create(custom_elements: &mut CustomElements, element: Element) {
        let shadow = customelement::attach_shadow(element);
        let id = custom_elements.add(ColorText {
            element: element,
            shadow: shadow,
        });

        let mut cb = create_event_listener();
        eventtarget::add_event_listener(element, "connected", cb);
        custom_elements.add_callback(
            cb,
            Box::new(move |custom_elements, _| {
                custom_elements.get::<ColorText>(id).connected();
            }),
        );

        cb = create_event_listener();
        eventtarget::add_event_listener(element, "attributechanged", cb);
        custom_elements.add_callback(
            cb,
            Box::new(move |custom_elements, event| {
                custom_elements
                    .get::<ColorText>(id)
                    .attribute_changed(event);
            }),
        );
    }

    fn connected(&self) {
        self.render();
    }

    fn attribute_changed(&self, _event: i32) {
        self.render();
    }
    fn render(&self) {
        let c = element::get_attribute(self.element, "color");
        element::set_inner_html(
            self.shadow,
            &format!(
                "<style>:host{{color:{} }}</style><div><slot></slot></div>",
                c
            ),
        );
    }
}

thread_local! {
    static CUSTOM_ELEMENTS:std::cell::RefCell<CustomElements> = std::cell::RefCell::new(CustomElements::new(
    |custom_elements, tag, element| match tag {
        "color-text" => ColorText::create(custom_elements, element),
        _ => console::error(&format!("unknown web component {}", tag)),
    }))
}

#[no_mangle]
pub fn main() -> () {
    // This function starts listening for hello-world components
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().define_with_attributes("color-text", "color");
    });
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: i32) {
    // This function routes callbacks to the right closure
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().route_callback(callback_id, event);
    });
}
