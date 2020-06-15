/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
 
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    fn js_log(msg: &str);

    #[wasm_bindgen(js_namespace = console, js_name = "warn")]
    fn js_warn(msg: &str);

    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    fn js_error(msg: &str);

    #[wasm_bindgen(js_namespace = console, js_name = "debug")]
    fn js_debug(msg: &str);
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum ApplicationType {
    ClientWeb,
    Wasm,
    Unknown,
}

// JS Logger
#[wasm_bindgen]
pub fn wasm_logger(level: LogLevel, app_type: Option<ApplicationType>, message: &str) {
    // get iso date, convert to String object
    let date_now = format!("{:?}", js_sys::Date::new_0().to_iso_string());

    // hack: remove the shitty double quotes!!!1
    let fmt_date = date_now.replace("\"", "");
    let selected_app_type = match app_type {
        Some(x) => x,
        None => ApplicationType::Unknown,
    };

    match level {
        LogLevel::Info => js_log(
            &format!(
                "[{}][{:#?}][{:#?}][{:#?}]",
                fmt_date,
                LogLevel::Info,
                selected_app_type,
                &message
            )
            .to_string(),
        ),
        LogLevel::Warn => js_warn(
            &format!(
                "[{}][{:#?}][{:#?}][{:#?}]",
                fmt_date,
                LogLevel::Warn,
                selected_app_type,
                &message
            )
            .to_string(),
        ),
        LogLevel::Error => js_error(
            &format!(
                "[{}][{:#?}][{:#?}][{:#?}]",
                fmt_date,
                LogLevel::Error,
                selected_app_type,
                &message
            )
            .to_string(),
        ),
        LogLevel::Debug => js_debug(
            &format!(
                "[{}][{:#?}][{:#?}][{:#?}]",
                fmt_date,
                LogLevel::Debug,
                selected_app_type,
                &message
            )
            .to_string(),
        ),
        // _ => js_log(&"default".to_string()),
    };
}
