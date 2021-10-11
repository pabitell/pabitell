use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

//#[wasm_bindgen(module = "/jsQR/dist/jsQR.js")]
#[wasm_bindgen]
extern "C" {
    pub fn jsQR(data: Clamped<Vec<u8>>, width: u32, height: u32, options: JsValue) -> JsValue;
}

pub fn js_qr(data: Clamped<Vec<u8>>, width: u32, height: u32, options: JsValue) -> Option<String> {
    let res = jsQR(data, width, height, options).into_serde().ok()?;
    match res {
        serde_json::Value::Object(obj) => {
            let data = obj.get("data")?;
            match data {
                serde_json::Value::String(data) => Some(data.to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}
