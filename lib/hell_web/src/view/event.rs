use std::collections::HashMap;

use hell_error::HellResult;
use wasm_bindgen::{prelude::Closure, JsCast};
use crate::error::ErrToWebHellErr;

use super::{Element, ElementTree};

// ---------------------------------------------------------------------------  -

#[derive(Debug)]
pub struct EventHandler {
    closure: Closure<dyn FnMut()>
}

impl EventHandler {
    pub fn new<F>(element: &mut Element, event_type: &str, callback: F) -> HellResult<Self>
    where F: FnMut() + 'static
    {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);

        element
            .inner()
            .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
            .unwrap();

        // NOTE: leaking memory
        // closure.forget();

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
