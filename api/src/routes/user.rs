use std::sync::{Arc, Mutex};
use poem::{handler, web::{Data, Json}};
use store::store::Store;

use crate::{requests_input::CreateUserInput, requests_output::{CreateUserOutput, SignInUserOutput}};

#[handler]
pub fn create_user(Json(data):Json<CreateUserInput>, Data(s):Data<&Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
    let mut ls = s.lock().unwrap();
    let id = ls.create_user(data.username, data.password).unwrap();
    let response = CreateUserOutput {
        id: id
    };

    Json(response)
}

#[handler]
pub fn get_user(Json(data):Json<CreateUserInput>, Data(s):Data<&Arc<Mutex<Store>>>) -> Json<SignInUserOutput> {
    let mut ls = s.lock().unwrap();
    let exists = ls.get_user(data.username, data.password).unwrap();
    if !exists {
        return Json(SignInUserOutput { jwt: "".to_string() })
    }
    let response = SignInUserOutput {
        jwt: String::from("trialjsonwebwebtoken")
    };

    Json(response)
}