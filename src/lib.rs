use wasm_bindgen::prelude::*;

use deutsche_bahn_delay_reasons;

#[wasm_bindgen]
pub fn get_grund() -> String {
    deutsche_bahn_delay_reasons::get_grund().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
