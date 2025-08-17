// use std::io::Error;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     println!("Hello, world!");
//     Ok(())
// }

use std::sync::{Arc, Mutex};

use dotenvy::dotenv;
use poem::{get, post, listener::TcpListener, EndpointExt, Route, Server};
use store::store::Store;

use crate::routes::{user::{create_user, get_user}, website::{create_website, get_website}};


pub mod requests_input;
pub mod requests_output;
pub mod routes;
pub mod auth_middleware;


#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::default()));
    dotenv().ok();

    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(create_user))
        .at("/user/signin", post(get_user))
        .data(s);
    Server::new(TcpListener::bind("0.0.0.0:3001"))
      .run(app)
      .await
}