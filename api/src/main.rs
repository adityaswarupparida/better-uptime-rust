// use std::io::Error;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     println!("Hello, world!");
//     Ok(())
// }

use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};
use store::store::Store;

use crate::{requests_input::{CreateUserInput, CreateWebsiteInput}, requests_output::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SignInUserOutput}};

pub mod requests_input;
pub mod requests_output;

#[handler]
fn create_user(Json(data):Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let mut s = Store::default();
    let id = s.create_user(data.username, data.password).unwrap();
    let response = CreateUserOutput {
        id: id
    };

    Json(response)
}

#[handler]
fn get_user(Json(data):Json<CreateUserInput>) -> Json<SignInUserOutput> {
    let mut s = Store::default();
    let exists = s.get_user(data.username, data.password).unwrap();
    if !exists {
        return Json(SignInUserOutput { jwt: "".to_string() })
    }
    let response = SignInUserOutput {
        jwt: String::from("trialjsonwebwebtoken")
    };

    Json(response)
}

#[handler]
fn get_website(Path(website_id): Path<String>) -> Json<GetWebsiteOutput> {
    // format!("hello: {}", website_id)
    let mut s = Store::default();
    let wb = s.get_website(website_id).unwrap();
    let response = GetWebsiteOutput {
        url: wb.url
    };

    Json(response) 
}

#[handler]
fn create_website(Json(data):Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    // format!("hello: {}", website_id)
    let url = data.url;
    let mut s = Store::default();
    let id = s.create_website(url, String::from("b13f8e5a-5be4-496b-8a4f-257e3f269f7f")).unwrap();
    let response = CreateWebsiteOutput {
        id: id
    };

    Json(response) 
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(create_user))
        .at("/user/signin", get(get_user));
    Server::new(TcpListener::bind("0.0.0.0:3001"))
      .run(app)
      .await
}