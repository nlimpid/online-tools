mod core;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn gen(name: &str) -> String {
    let inner_result = core::parse_sql(name.to_string());
    return core::gen_go_code(inner_result.unwrap());
    match inner_result {
        Ok(res) => {
            let got = core::gen_go_code(res);
            return got;
        }
        Err(e) => {
            println!("err {}", e);
            return "".to_string();
        }
    }
}
