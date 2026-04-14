use wasm_bindgen::prelude::*;

mod player_character;

#[wasm_bindgen]
pub fn greet() -> String {
    "Rules engine loaded.".to_string()
}

// --- your real code above ---

#[cfg(test)]        // "only compile this block when running tests"
mod tests {         // a module = a namespace, like a Python package or Apex class

    #[test]         // marks this function as a test case
    fn my_test() {
        assert_eq!(2 + 2, 4);   // panics (fails) if not equal
    }
}