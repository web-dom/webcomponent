# webcomponent
A simple web component system for Rust using [wasm-module](https://github.com/richardanaya/wasm-module) for DOM access.

Let's first create a component `<hello-world>` that simply sets its inner HTML to "Hello World"

```rust
pub struct HelloWorld {}

impl HelloWorld {
    pub fn create(element: Element) {
        unsafe {
            Element_set_innerHTML(element,cstr("Hello World")
        }
    }
}
```

Register new custom element

```rust
CustomElement_define(cstr("hello-world"));
```

Listen for when a new instance of that custom element is constructed in the DOM and create an associated element in Rust to manage it.

```rust
let window = global_getWindow();
let callbackHandle = global_createEventListener();
EventTarget_addEventListener(window, cstr("customelementcreated"), callbackHandle);
add_callback(
    callbackHandle,
    Box::new(|event| {
        let element = global_getProperty(event, cstr("detail"));
        HelloWorld::create(element);
    }),
);
```

Putting it all together

```toml
[package]
name = "helloworld"

[lib]
crate-type =["cdylib"]

[dependencies]
webcomponent = "0.1"
```

```rust
use webcomponent::*;

extern "C" {
    pub fn global_getWindow() -> Element;
    pub fn global_createEventListener() -> Element;
    pub fn global_getProperty(obj: Element, name: CString) -> Element;
    pub fn EventTarget_addEventListener(element: Element, eventName: CString, callback: Callback);
    pub fn Element_set_innerHTML(element: Element, text: CString);
    pub fn CustomElement_define(name: CString);
}

pub struct HelloWorld {}

impl HelloWorld {
    pub fn create(element: Element) {
        unsafe {
            Element_set_innerHTML(element, cstr("Hello World"));
        }
    }
}

#[no_mangle]
pub fn main() -> () {
    unsafe {
        let window = global_getWindow();
        let callbackHandle = global_createEventListener();
        EventTarget_addEventListener(window, cstr("customelementcreated"), callbackHandle);
        add_callback(
            callbackHandle,
            Box::new(|event| {
                let element = global_getProperty(event, cstr("detail"));
                HelloWorld::create(element);
            }),
        );
        CustomElement_define(cstr("hello-world"));
    }
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: i32) {
    // this function routes callbacks to the right closure
    route_callback(callback_id, event);
}
```

Compile and load the web assembly module using [wasm-module](https://github.com/richardanaya/wasm-module)

```console
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/helloworld.wasm .
```

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <script src="https://unpkg.com/@webcomponents/webcomponentsjs@latest/webcomponents-loader.js"></script>
    <script src="https://unpkg.com/wasm-module@latest/wasm-module.js"></script>
  </head>
  <body>
    <hello-world></hello-world>
    <wasm-module src="helloworld.wasm"></wasm-module>
  </body>
</html>
```

See it working [here](https://richardanaya.github.io/webcomponent/examples/helloworld/)



# Let's make a clock

In order to make a clock we'll need to be able to hold onto our component at a global level so it doesn't get deallocated. Also we'll have to hold onto the DOM element associated with our component. To help with these are two global storage component utility functions:
* get_components() - to get a global mutable vector of components of a particular type
* get_component(x) - get get a specific member by index of the vector of components of a particular type

For example: `get_components::<XClock>()` will get me a global vector of all known XClock components

```rust
struct XClock {
    element: i32,
}

impl XClock {
    fn create(element: i32) {
        unsafe {
            let x = XClock { element: element };
            x.render();

            // store xclock and keep its index
            get_components().push(x);
            let id = get_components::<XClock>().len() - 1;

            let cb = global_createEventListener();
            let window = global_getWindow();
            Window_setInterval(window, cb, 1000);
            add_callback(
                cb,
                Box::new(move |\_| {
                    get_component::<XClock>(id).timer();
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

```
CustomElement_define(cstr("x-clock"));
```

See it working [here](https://richardanaya.github.io/webcomponent/examples/xclock/)
