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
// listen to custom element creation on the window
let win = global_getWindow();
let cb = global_createEventListener();
EventTarget_addEventListener(win, cstr("customelementcreated"), cb);
add_callback( cb, Box::new(|event| {
        // event "detail" property contains a handle to the new custom element
        let element = global_getProperty(event, cstr("detail"));
        HelloWorld::create(element);
    }),
);
```

Putting it all together

```rust
use webcomponent::*;

extern "C" {
    pub fn global_getWindow() -> Element;
    pub fn global_createEventListener() -> Element;
    pub fn global_getProperty(obj: Element, name: CString) -> Element;
    pub fn EventTarget_addEventListener(element: Element, eventName: CString, callback: Callback);
    pub fn Element_set_innerHTML(element: Element, text: CString);
    pub fn CustomElement_define(name: CString,attributes: CString);
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
        let win = global_getWindow();
        let cb = global_createEventListener();
        EventTarget_addEventListener(win, cstr("customelementcreated"), cb);
        add_callback(
            cb,
            Box::new(|event| {
                let element = global_getProperty(event, cstr("detail"));
                HelloWorld::create(element);
            }),
        );
        CustomElement_define(cstr("hello-world"),cstr("blah"));
    }
}

#[no_mangle]
pub fn callback(callback_id: Callback, event: i32) {
    // this function routes callbacks to the right closure
    route_callback(callback_id, event);
}
```

Compile and load the web assembly module using [wasm-module](https://github.com/richardanaya/wasm-module)

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
