/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
 
mod models;
mod utils;

use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// data mapper
fn map_userstruct_to_simp(src: &models::UserStruct) -> models::SimpleUser {
    let mut user = models::SimpleUser::new();

    // user.set_id(src.id.value.unwrap());
    user.set_name(
        src.name.title.to_string(),
        src.name.first.to_string(),
        src.name.last.to_string(),
    );
    user.set_username(src.login.username.to_string());
    user.set_password(src.login.password.to_string());
    user.set_email(src.email.to_string());
    user.set_email(src.phone.to_string());
    user.set_location(
        src.location.city.to_string(),
        src.location.state.to_string(),
        src.location.country.to_string(),
    );
    user.set_image_thumb(src.picture.thumbnail.to_string());
    user.set_image_mid(src.picture.medium.to_string());
    user.set_image_lrg(src.picture.large.to_string());
    user
}

// private base get_user function
async fn get_user_base(url: String) -> Result<Vec<models::UserStruct>, Error> {
    utils::set_panic_hook();

    utils::wasm_logger(
        utils::LogLevel::debug,
        Some(utils::ApplicationType::Wasm),
        &format!("inside get_user_base. url={:?}", url),
    );

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    let data: models::RandomuserResponse = json.into_serde().unwrap();

    Ok(data.results)
}

//
// public wasm function: map to Array of SimpleUser
// use v1.3 of the api
//
#[wasm_bindgen]
pub async fn get_user_wasm(
    gender: utils::Gender,
    nationality: utils::Nationality,
    limit: u32,
) -> Result<JsValue, JsValue> {
    utils::wasm_logger(
        utils::LogLevel::debug,
        Some(utils::ApplicationType::Wasm),
        &format!(
            "inside get_user_wasm. gender={:#?} nationality={:#?} limit={:?}",
            &gender, &nationality, &limit
        ),
    );

    let uri = format!(
        "https://randomuser.me/api/?gender={:#?}&nat={:#?}&results={}",
        gender, nationality, limit
    );
    utils::wasm_logger(
        utils::LogLevel::debug,
        Some(utils::ApplicationType::Wasm),
        &format!("get_user_wasm final url: {:#?}", &uri),
    );

    let data = get_user_base(uri).await?;

    let output: Vec<models::SimpleUser> = data
        .iter()
        .map(|item| map_userstruct_to_simp(item))
        .collect();

    utils::wasm_logger(
        utils::LogLevel::debug,
        Some(utils::ApplicationType::Wasm),
        serde_json::to_string(&output).unwrap().as_str(),
    );

    // need to fix this - returned as Any in Typescript
    Ok(JsValue::from_serde(&output).unwrap())
}
