// use std::io::Error;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     println!("Hello, world!");
//     Ok(())
// }

use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};
use store::Store;

use crate::{requests_input::CreateWebsiteInput, requests_output::CreateWebsiteOutput};

pub mod requests_input;
pub mod requests_output;


#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello: {}", website_id)
}

#[handler]
fn create_website(Json(data):Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    // format!("hello: {}", website_id)
    let url = data.url;
    let s = Store{};
    s.create_website();
    let response = CreateWebsiteOutput {
        id: url
    };

    Json(response) 
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3001"))
      .run(app)
      .await
}