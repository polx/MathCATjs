use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;
use libmathcat::*;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

#[macro_use]
extern crate log;




#[wasm_bindgen(js_name = "mathmlToSpeech")]
pub fn mathml_to_speech(mathml: &str) -> String{
    set_mathml(String::from(mathml));
    return match get_spoken_text() {
        Ok(text) => text,
        Err(e) => errors_to_string(&e),
    };
}

pub static mut INITTED: bool = false;

#[wasm_bindgen(js_name = "initMathCAT")]
pub unsafe fn init_math_cat(prefs: &JsValue ) {
    if !INITTED {
        if let Err(e) = set_rules_dir("Rules".to_string()) {
            error!("Didn't find rules dir: {}", e.to_string());
        };
        INITTED = true;
    }
    if prefs.is_string() {
        set_preference("Language".to_string(), prefs.as_string().unwrap()).unwrap()
    } else if prefs.is_object() {
        for k in js_sys::Reflect::own_keys(prefs).unwrap() {
            let key = k.as_string().unwrap();
            let value = js_sys::Reflect::get(prefs, k.as_ref()).unwrap().as_string().unwrap();
            info!("Setting pref {}: '{}'", key, value);
            set_preference(key, value).unwrap()
        }
    }
}
