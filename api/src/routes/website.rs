use std::sync::{Arc, Mutex};
use poem::{handler, web::{Data, Json, Path}};
use store::store::Store;

use crate::{requests_input::CreateWebsiteInput, requests_output::{CreateWebsiteOutput, GetWebsiteOutput}};

#[handler]
pub fn get_website(Path(website_id): Path<String>, Data(s):Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
    // format!("hello: {}", website_id)
    let mut ls = s.lock().unwrap();
    let wb = ls.get_website(website_id).unwrap();
    let response = GetWebsiteOutput {
        url: wb.url
    };

    Json(response) 
}

#[handler]
pub fn create_website(Json(data):Json<CreateWebsiteInput>, Data(s):Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteOutput> {
    // format!("hello: {}", website_id)
    let url = data.url;
    let mut ls = s.lock().unwrap();
    let id = ls.create_website(url, String::from("b13f8e5a-5be4-496b-8a4f-257e3f269f7f")).unwrap();
    let response = CreateWebsiteOutput {
        id: id
    };

    Json(response) 
}