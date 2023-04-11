use wasm_bindgen::prelude::wasm_bindgen;
extern crate console_error_panic_hook;

pub fn init_logging() {
    // use std::panic;
    // panic::set_hook(Box::new(console_error_panic_hook::hook));

    console_error_panic_hook::set_once()
}

// ----------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        // HACK (lm): unsafe block only exists to make rust-analyzer shut up about this macro being unsafe
        #[allow(unused)]
        unsafe {
            $crate::logging::log(&format_args!($($t)*).to_string())
        }
    };
}

