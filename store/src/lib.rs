// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

use std::env;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
use diesel::prelude::*;

pub struct Store {
    conn: PgConnection
}

impl Default for Store {
    fn default() -> Self {
        Self { conn: self::establish_connection() }
    }
}

impl Store {
    pub fn create_user(&self) {
        print!("create user called")
    }
    pub fn create_website(&self) -> String {
        String::from("1")
    }
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}