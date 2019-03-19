# webcomponent
A simple web component system for Rust using [wasm-module](https://github.com/richardanaya/wasm-module) for DOM access.

Let's first create a component `<hello-world>` that simply sets its inner HTML to "Hello World"

```rust
pub struct HelloWorld {}

impl HelloWorld {
    pub fn create(custom_elements: &CustomElements, element: Element) {
        unsafe {
            Element_set_innerHTML(element,cstr("Hello World")
        }
    }
}
```

See it working [here](https://richardanaya.github.io/webcomponent/examples/helloworld/)



# Let's make a clock

In order to make a clock we'll need to be able to hold onto our component at a global level so it doesn't get deallocated.

```rust
struct XClock {
    element: i32,
}

impl XClock {
    fn create(custom_elements: &CustomElements, element: i32) {
        unsafe {
            let x = XClock { element: element };
            x.render();

            let id = custom_elements.add(x);

            let cb = global_createEventListener();
            let window = global_getWindow();
            Window_setInterval(window, cb, 1000);
            custom_elements.add_callback(
                cb,
                Box::new(move |custom_elements,event| {
                    custom_elements.get::<XClock>(id).timer();
                }),
            );

        }
    }

    fn timer(&self) {
        self.render();
    }

    fn render(&self){
        unsafe {
            let d = Date_nowSeconds();
            let o = Date_getTimezoneOffset();
            let now: DateTime<Utc> = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp((d-(o*60)) as i64, 0), Utc);
            Element_set_innerHTML(self.element,cstr(&format!("{}",now.format("%I:%M:%S %p"))));
        }
    }
}
```

See it working [here](https://richardanaya.github.io/webcomponent/examples/xclock/)

# Observing Attributes

Let's take a look at an example that takes advantage of observing attribute changes and also a bit of shadow DOM.

```rust
pub struct ColorText {
    element: Element,
    shadow: Element,
}

impl ColorText {
    fn create(custom_elements: &CustomElements, element: Element) {
        unsafe {
            let shadow = Element_attachShadow(element);
            let id = custom_elements.add(ColorText {
                element: element,
                shadow: shadow,
            });

            let mut cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("connected"), cb);
            custom_elements.add_callback(
                cb,
                Box::new(move |custom_elements,event| {
                    custom_elements.get::<ColorText>(id).connected();
                }),
            );

            cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("attributechanged"), cb);
            custom_elements.add_callback(
                cb,
                Box::new(move |custom_elements,event| {
                    custom_elements.get::<ColorText>(id).attribute_changed(event);
                }),
            );
        }
    }

    fn connected(&self) {
        self.render();
    }

    fn attribute_changed(&self, _event: i32) {
        self.render();
    }

    fn render(&self) {
        unsafe {
            let c = Element_getAttribute(self.element, cstr("color"));
            Element_set_innerHTML(
                self.shadow,
                cstr(&format!(
                    "<style>:host{{color:{} }}</style><div><slot></slot></div>",
                    cstr_to_string(c)
                )),
            );
        }
    }
}
```

See it working [here](https://richardanaya.github.io/webcomponent/examples/colortext/)
