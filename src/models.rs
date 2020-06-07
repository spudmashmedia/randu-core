/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

//!domain models for randomuser.me/api response
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

//-------------------------------------------------------
// Struct:      InfoStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct InfoStruct {
    pub seed: String,
    pub results: u32,
    pub page: u32,
    pub version: String,
}

//-------------------------------------------------------
// Struct:      NameStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct NameStruct {
    pub title: String,
    pub first: String,
    pub last: String,
}
// impl NameStruct {
//     pub fn PrettyName(&self) {
//         let pretty_name = format!("{title} {first} {last}", self.title, self.first, self.last);
//         pretty_name
//     }
// }

//-------------------------------------------------------
// Struct:      LoginStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStruct {
    pub uuid: Uuid,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
}

//-------------------------------------------------------
// Struct:      DobStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct DobStruct {
    pub date: String,
    pub age: u32,
}

//-------------------------------------------------------
// Struct:      IdStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct IdStruct {
    pub name: String,
    pub value: serde_json::value::Value, //unknown value
}

//-------------------------------------------------------
// Struct:      PictureStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct PictureStruct {
    pub large: String,
    pub medium: String,
    pub thumbnail: String,
}

//-------------------------------------------------------
// Struct:      StreetStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct StreetStruct {
    pub number: u32,
    pub name: String,
}

//-------------------------------------------------------
// Struct:      GpsStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct GpsStruct {
    pub latitude: String,
    pub longitude: String,
}

//-------------------------------------------------------
// Struct:      TimezoneStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct TimezoneStruct {
    pub offset: String,
    pub description: String,
}

//-------------------------------------------------------
// Struct:      LocationStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationStruct {
    pub street: StreetStruct,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postcode: serde_json::value::Value, // NOTE: can be either String or u32 depending on region
    pub coordinates: GpsStruct,
    pub timezone: TimezoneStruct,
}

//-------------------------------------------------------
// Struct:      UserStruct
// Description:
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct UserStruct {
    pub gender: String,
    pub name: NameStruct,
    pub location: LocationStruct,
    pub email: String,
    pub login: LoginStruct,
    pub dob: DobStruct,
    pub registered: DobStruct,
    pub phone: String,
    pub cell: String,
    pub id: IdStruct,
    pub picture: PictureStruct,
    pub nat: String,
}

//-------------------------------------------------------
// Struct:      RandomuserResponse
// Description: wrapper payload
//-------------------------------------------------------
#[derive(Serialize, Deserialize, Debug)]
pub struct RandomuserResponse {
    pub results: Vec<UserStruct>,
    pub info: InfoStruct,
}


#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleUser {
    id: String,
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
            id: String::new(),
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
        id: String,
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
            id: id,
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

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn set_id(&mut self, val: String) {
        self.id = val;
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
