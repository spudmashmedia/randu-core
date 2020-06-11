/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
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

