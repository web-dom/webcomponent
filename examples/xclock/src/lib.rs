use webcomponent::*;
extern crate chrono;
use chrono::{DateTime, NaiveDateTime, Utc};

struct XClock {
    element: i32,
}

impl XClock {
    fn create(custom_elements: &mut CustomElements, element: i32) {
        let x = XClock { element: element };
        x.render();
        let id = custom_elements.add(x);
        let cb = create_event_listener();
        window::set_interval(window(), cb, 1000);
        custom_elements.add_callback(
            cb,
            Box::new(move |custom_elements, _event| {
                custom_elements.get::<XClock>(id).timer();
            }),
        );
    }

    fn timer(&self) {
        self.render();
    }

    fn render(&self) {
        let d = date::now_seconds();
        let o = date::get_timezone_offset();
        let now: DateTime<Utc> =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp((d - (o * 60)) as i64, 0), Utc);
        element::set_inner_html(self.element, &format!("{}", now.format("%I:%M:%S %p")));
    }
}

thread_local! {
    static CUSTOM_ELEMENTS:std::cell::RefCell<CustomElements> = std::cell::RefCell::new(CustomElements::new(
    |custom_elements, tag, element| match tag {
        "x-clock" => XClock::create(custom_elements, element),
        _ => console::error(&format!("unknown web component {}", tag)),
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
