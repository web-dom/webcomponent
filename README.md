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

```
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

```
<!DOCTYPE html>
<html lang="en">
  <head>
    <script src="https://unpkg.com/@webcomponents/webcomponentsjs@latest/webcomponents-loader.js"></script>
    <script src="https://unpkg.com/wasm-module@latest/wasm-module.min.js"></script>
  </head>
  <body>
    <hello-world></hello-world>
    <wasm-module src="helloworld.wasm"></wasm-module>
  </body>
</html>
```
