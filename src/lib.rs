/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

mod models;
mod utils;

use js_sys::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};


//---------------------------------------------------------------------------
// STRUCTS & TYPES
//---------------------------------------------------------------------------

#[wasm_bindgen]
#[derive(Debug)]
pub enum Gender {
    male,
    female,
    unknown,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum Nationality {
    au,
    br,
    ca,
    ch,
    de,
    dk,
    es,
    fi,
    fr,
    gb,
    ie,
    ir,
    no,
    nl,
    nz,
    tr,
    us,
}

#[wasm_bindgen]
#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct SimpleUser {
    name: String,
    username: String,
    password: String,
    email: String,
    phone: String,
    location: String,
    image_thumb: String,
    image_mid: String,
    image_lrg: String,
}

#[wasm_bindgen(js_name = SimpleUser)]
impl SimpleUser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SimpleUser {
        SimpleUser {
            name: String::new(),
            username: String::new(),
            password: String::new(),
            email: String::new(),
            phone: String::new(),
            location: String::new(),
            image_thumb: String::new(),
            image_mid: String::new(),
            image_lrg: String::new(),
        }
    }

    pub fn new_set(
        name: String,
        username: String,
        password: String,
        email: String,
        phone: String,
        location: String,
        image_thumb: String,
        image_mid: String,
        image_lrg: String,
    ) -> SimpleUser {
        SimpleUser {
            name: name,
            username: username,
            password: password,
            email: email,
            phone: phone,
            location: location,
            image_thumb: image_thumb,
            image_mid: image_mid,
            image_lrg: image_lrg,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub fn set_name(&mut self, title: String, first_name: String, last_name: String) {
        self.name.clear();
        let temp = format!(
            "{title} {first} {last}",
            title = title,
            first = first_name,
            last = last_name
        );

        self.name.push_str(&temp.to_string());
    }

    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    pub fn set_username(&mut self, val: String) {
        self.username = val;
    }

    pub fn get_password(&self) -> String {
        self.password.to_string()
    }

    pub fn set_password(&mut self, val: String) {
        self.password = val;
    }

    pub fn get_email(&self) -> String {
        self.email.to_string()
    }

    pub fn set_email(&mut self, val: String) {
        self.email = val;
    }

    pub fn get_phone(&self) -> String {
        self.phone.to_string()
    }

    pub fn set_phone(&mut self, val: String) {
        self.phone = val;
    }

    pub fn get_location(&self) -> String {
        self.location.to_string()
    }

    pub fn set_location(&mut self, city: String, state: String, country: String) {
        self.location = format!("{}, {} {}", city, state, country);
    }

    pub fn get_image_thumb(&self) -> String {
        self.image_thumb.to_string()
    }

    pub fn set_image_thumb(&mut self, val: String) {
        self.image_thumb = val;
    }

    pub fn get_image_mid(&self) -> String {
        self.image_mid.to_string()
    }

    pub fn set_image_mid(&mut self, val: String) {
        self.image_mid = val;
    }

    pub fn get_image_lrg(&self) -> String {
        self.image_lrg.to_string()
    }

    pub fn set_image_lrg(&mut self, val: String) {
        self.image_lrg = val;
    }
}

//---------------------------------------------------------------------------
// FUNCTIONS
//---------------------------------------------------------------------------

// data mapper
fn map_userstruct_to_simp(src: &models::UserStruct) -> SimpleUser {
    let mut user = SimpleUser::new();

    user.set_name(
        src.name.title.to_string(),
        src.name.first.to_string(),
        src.name.last.to_string(),
    );
    user.set_username(src.login.username.to_string());
    user.set_password(src.login.password.to_string());
    user.set_email(src.email.to_string());
    user.set_phone(src.phone.to_string());
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
    gender: Gender,
    nationality: Nationality,
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
        "https://randomuser.me/api/1.3/?gender={:#?}&nat={:#?}&results={}",
        gender, nationality, limit
    );
    utils::wasm_logger(
        utils::LogLevel::debug,
        Some(utils::ApplicationType::Wasm),
        &format!("get_user_wasm final url: {:#?}", &uri),
    );

    let data = get_user_base(uri).await?;

    let output: Vec<SimpleUser> = data
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

// LOAD UNIT TESTS
#[cfg(test)]
mod lib_tests;
