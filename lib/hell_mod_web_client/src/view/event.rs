use hell_core::error::HellResult;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Event;

// ---------------------------------------------------------------------------  -

pub struct EventHandlerId(usize);

impl EventHandlerId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub struct EventHandler {
    closure: Closure<dyn FnMut(Event)>
}

impl EventHandler {
    pub fn from_event<C>(element: &web_sys::Element, event_type: &str, cb: C) -> HellResult<Self>
    where C: FnMut(Event) + 'static
    {
        let closure = Closure::wrap(Box::new(cb) as Box<dyn FnMut(Event)>);

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
