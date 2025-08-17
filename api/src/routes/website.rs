use std::sync::{Arc, Mutex};
use poem::{handler, web::{Data, Json, Path}};
use store::store::Store;

use crate::{auth_middleware::UserId, requests_input::CreateWebsiteInput, requests_output::{CreateWebsiteOutput, GetWebsiteOutput}};

#[handler]
pub fn get_website(Path(website_id): Path<String>, Data(s):Data<&Arc<Mutex<Store>>>, UserId(user_id): UserId) -> Json<GetWebsiteOutput> {
    // format!("hello: {}", website_id)
    let mut ls = s.lock().unwrap();
    let wb = ls.get_website(website_id, user_id).unwrap();
    let response = GetWebsiteOutput {
        url: wb.url,
        id: wb.id,
        user_id: wb.user_id
    };

    Json(response) 
}

#[handler]
pub fn create_website(Json(data):Json<CreateWebsiteInput>, Data(s):Data<&Arc<Mutex<Store>>>, UserId(user_id): UserId) -> Json<CreateWebsiteOutput> {
    // format!("hello: {}", website_id)
    let url = data.url;
    let mut ls = s.lock().unwrap();
    let id = ls.create_website(url, user_id).unwrap();
    let response = CreateWebsiteOutput {
        id
    };

    Json(response) 
}