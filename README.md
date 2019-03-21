# webcomponent
A simple web component system for Rust using [web-dom](https://github.com/web-dom/web-dom) for DOM access.

```toml
webcomponent = "0.3"
```

Let's first create a component `<hello-world>` that simply sets its inner HTML to "Hello World"

```rust
pub struct HelloWorld {}
impl HelloWorld {
    pub fn create(_custom_elements: &CustomElements, element: Element) {
        element::set_inner_html(element, "Hello World!");
    }
}
```

Now lets do some setup to register this custom element and setup a routing system for events from DOM

```rust
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
pub fn callback(callback_id: EventListener, event: Event) {
    // This function routes callbacks to the right closure
    CUSTOM_ELEMENTS.with(|c| {
        c.borrow_mut().route_callback(callback_id, event);
    });
}
```

See it working [here](https://web-dom.github.io/webcomponent/examples/helloworld/)



# Let's make a clock

In order to make a clock we'll need to be able to hold onto our component at a global level so it doesn't get deallocated.

```rust
struct XClock {
    element: Element,
}

impl XClock {
    fn create(custom_elements: &mut CustomElements, element: Element) {
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
```

See it working [here](https://web-dom.github.io/webcomponent/examples/xclock/)

# Observing Attributes

Let's take a look at an example that takes advantage of observing attribute changes and also a bit of shadow DOM.

```rust
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

    fn attribute_changed(&self, _event: Event) {
        self.render();
    }

    fn render(&self) {
        let c = element::get_attribute(self.element, "color");
        element::set_inner_html(
            self.shadow,
            &format!(
                "<style>:host\{color:{} \}</style><div><slot></slot></div>",
                c
            ),
        );
    }
}
```

See it working [here](https://web-dom.github.io/webcomponent/examples/colortext/)
