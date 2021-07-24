use wasm_bindgen::prelude::*;

use deutsche_bahn_delay_reasons;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn alert_reason() {
    alert(&format!(
        "Grund: {}",
        deutsche_bahn_delay_reasons::get_grund()
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
