use super::*;

use serde_json::*;
use uuid::Uuid;

#[test]
fn test_map_userstruct_to_simp() {
    // ararnge
    let test_data = models::UserStruct {
        gender: "male".to_string(),
        name: models::NameStruct {
            title: "Mr".to_string(),
            first: "Bubba".to_string(),
            last: "Ganoush.to_string()".to_string(),
        },
        location: models::LocationStruct {
            street: models::StreetStruct {
                number: 42,
                name: "Pitt Street".to_string(),
            },
            city: "Sydney".to_string(),
            state: "NSW".to_string(),
            country: "Australia".to_string(),
            postcode: json!(2000),
            coordinates: models::GpsStruct {
                latitude: "0".to_string(),
                longitude: "0".to_string(),
            },
            timezone: models::TimezoneStruct {
                offset: "+10".to_string(),
                description: "Sydney".to_string(),
            },
        },
        email: "a_random@email.com".to_string(),
        login: models::LoginStruct {
            uuid: Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            username: "String".to_string(),
            password: "String".to_string(),
            salt: "String".to_string(),
            md5: "String".to_string(),
            sha1: "String".to_string(),
            sha256: "String".to_string(),
        },
        dob: models::DobStruct {
            date: "01/01/1900".to_string(),
            age: 120,
        },
        registered: models::DobStruct {
            date: "01/01/1900".to_string(),
            age: 120,
        },
        phone: "+61414000111".to_string(),
        cell: "".to_string(),
        id: models::IdStruct {
            name: "bubba_ganoush".to_string(),
            value: json!(1),
        },
        picture: models::PictureStruct {
            large: "http://large".to_string(),
            medium: "http://medium".to_string(),
            thumbnail: "http://thumb".to_string(),
        },
        nat: "au".to_string(),
    };

    let expected_response = models::SimpleUser::new_set(
        format!(
            "{title} {first} {last}",
            title = &test_data.name.title,
            first = &test_data.name.first,
            last = &test_data.name.last
        ),
        format!("{}", &test_data.login.username),
        format!("{}", &test_data.login.password),
        format!("{}", &test_data.email),
        format!("{}", &test_data.phone),
        
            format!(
                "{}, {} {}",
                &test_data.location.city, &test_data.location.state, &test_data.location.country
            ),
        format!("{}", &test_data.picture.thumbnail),
        format!("{}", &test_data.picture.medium),
         format!("{}", &test_data.picture.large),
    );

    // act & assert
    assert_eq!(map_userstruct_to_simp(&test_data), expected_response);
}
