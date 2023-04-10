use hell_error::HellResult;
use web_sys::{MessageEvent, ErrorEvent};
use wasm_bindgen::{JsCast, prelude::Closure};
use hell_web::console;


pub struct WSWebClient {
    ws: web_sys::WebSocket,
}

// fn console::log(s: impl Into<String>) {
//     let s: String = s.into();
//     web_sys::console::log_1(&s.into())
// }

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WsData {
    txt: String,
}

impl WSWebClient {
    pub fn new() -> HellResult<Self> {
        let ws = web_sys::WebSocket::new("ws://127.0.0.1:7666").unwrap();
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let onmessage_cb  = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                console::log!("message event, received arraybuffer: {:?}", abuf);
                let array = js_sys::Uint8Array::new(&abuf);
                let len = array.byte_length() as usize;
                console::log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());

                let array_s = array.to_string().as_string().unwrap();
                let data: WsData = serde_json::from_str(&array_s).unwrap();
                let data_str = serde_json::to_string(&data).unwrap();

                console::log!("DATA,: {}", data_str);

            } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                console::log!("message event, received blob: {:?}", blob);
                // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
                let fr = web_sys::FileReader::new().unwrap();
                let fr_c = fr.clone();
                // create onLoadEnd callback
                let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                    let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                    let len = array.byte_length() as usize;
                    console::log!("Blob received {}bytes: {:?}", len, array.to_vec());
                    // here you can for example use the received image/png data
                });
                fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                fr.read_as_array_buffer(&blob).expect("blob not readable");
                onloadend_cb.forget();
            }
            else {
                console::log!("message event, received Unknown: {:?}", e.data());
            }
        });

        ws.set_onmessage(Some(onmessage_cb.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_cb.forget();

        let onerror_cb = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
            console::log!("error event: {:?}", e);
        });

        ws.set_onerror(Some(onerror_cb.as_ref().unchecked_ref()));
        onerror_cb.forget();

        let cloned_ws = ws.clone();
        let onopen_cb = Closure::<dyn FnMut()>::new(move || {
            console::log!("socket opened");

            match cloned_ws.send_with_str("ping") {
                Ok(_) => console::log!("message successfully sent"),
                Err(err) => console::log!("error sending message: {:?}", err),
            }

            match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
                Ok(_) => console::log!("binary message successfully sent"),
                Err(err) => console::log!("error sending binary message: {:?}", err),
            }

        });

        ws.set_onopen(Some(onopen_cb.as_ref().unchecked_ref()));
        onopen_cb.forget();

        Ok(Self {
            ws
        })
    }
}
