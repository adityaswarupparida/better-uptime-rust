use std::sync::{Arc, Mutex};
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{handler, http::StatusCode, web::{Data, Json}, Error};
use serde::{Deserialize, Serialize};
use store::store::Store;

use crate::{requests_input::CreateUserInput, requests_output::{CreateUserOutput, SignInUserOutput}};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[handler]
pub fn create_user(Json(data):Json<CreateUserInput>, Data(s):Data<&Arc<Mutex<Store>>>) -> Result<Json<CreateUserOutput>, Error> {
    let mut ls = s.lock().unwrap();
    let id = ls.create_user(data.username, data.password)
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;
    let response = CreateUserOutput {
        id
    };

    Ok(Json(response))
}

#[handler]
pub fn get_user(Json(data):Json<CreateUserInput>, Data(s):Data<&Arc<Mutex<Store>>>) -> Result<Json<SignInUserOutput>, Error> {
    let mut ls = s.lock().unwrap();
    let results = ls.get_user(data.username, data.password);
    match results {
        Ok(user_id) => {
            let my_claims = Claims{
                sub: user_id,
                exp: 1111111
            };
            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
                .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;
            let response = SignInUserOutput {
                jwt: token
            };
            Ok(Json(response))
        }
        Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED))
    }

}