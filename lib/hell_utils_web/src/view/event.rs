use hell_core::error::HellResult;
use wasm_bindgen::{prelude::Closure, JsCast};

// ---------------------------------------------------------------------------  -

pub struct EventHandlerId(usize);

impl EventHandlerId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub struct EventHandler {
    closure: Closure<dyn FnMut()>
}

impl EventHandler {
    pub fn new<F>(element: &web_sys::Element, event_type: &str, callback: F) -> HellResult<Self>
    where F: FnMut() + 'static
    {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);

        element
            .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
            .unwrap();

        Ok(Self {
            closure,
        })
    }

    pub fn closure_function(&self) -> &js_sys::Function {
        self.closure.as_ref().unchecked_ref()
    }

    // NOTE: leaks memory
    pub fn forget(self) {
        self.closure.forget()
    }
}
