use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;

pub struct Store {
    pub conn: PgConnection
}

impl Default for Store {
    fn default() -> Self {
        let conn = establish_connection();
        Self { conn  }
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}