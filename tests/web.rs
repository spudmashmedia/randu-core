/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

wasm_bindgen_test_configure!(run_in_browser);

extern crate wasm_bindgen_test;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

extern crate randu_core;
use randu_core::{get_user_wasm, Gender, Nationality, SimpleUser};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    pub fn test_log(msg: &str);
}

#[wasm_bindgen_test]
async fn test_get_user_australia() {
    let expected_count: u32 = 1;
    let expected_nationality_string = "australia";

    let results: Vec<String> =
        test_get_user_base(Gender::Male, Nationality::AU, expected_count).await;

    assert_eq!(results.iter().count(), expected_count as usize);

    for current_location in results.iter() {
        assert!(current_location
            .to_lowercase()
            .contains(expected_nationality_string));
    }
}

#[wasm_bindgen_test]
async fn test_get_user_brazil() {
    let expected_count: u32 = 2;
    let expected_nationality_string = "brazil";

    let results: Vec<String> =
        test_get_user_base(Gender::Female, Nationality::BR, expected_count).await;

    assert_eq!(results.iter().count(), expected_count as usize);

    for current_location in results.iter() {
        assert!(current_location
            .to_lowercase()
            .contains(expected_nationality_string));
    }
}

// common test code
async fn test_get_user_base(
    test_gender: Gender,
    test_nationality: Nationality,
    test_count: u32,
) -> Vec<String> {
    let sut = get_user_wasm(test_gender, test_nationality, test_count);
    let promise = js_sys::Promise::resolve(&sut.await.unwrap());
    let result = JsFuture::from(promise).await.unwrap();

    // check if something returned
    assert_ne!(result, JsValue::NULL);

    // cast JsValue object to Rust object
    let actual_results: Vec<SimpleUser> = result.into_serde().unwrap();

    // check array count
    assert_eq!(actual_results.iter().count(), test_count as usize);

    // make sure expected country for each item
    let location_array: Vec<String> = actual_results.iter().map(|x| x.get_location()).collect();

    location_array
}
