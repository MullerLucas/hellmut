use wasm_bindgen::{JsValue, JsCast};
use web_sys::{RequestInit, RequestMode, Request, Response};
use wasm_bindgen_futures::JsFuture;

use crate::console_info;

#[derive(Debug)]
pub struct FetchApi;

impl FetchApi {
    pub async fn get() -> JsValue {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let url = "/models";
        let request = Request::new_with_str_and_init(url, &opts).unwrap();
        request.headers()
            .set("Accept", "application/json").unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        console_info!("RESPONSE: {:?}", resp);

        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        json
    }

}

