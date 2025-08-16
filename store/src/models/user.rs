use diesel::{prelude::*};
use uuid::Uuid;
use crate::{schema::user, store::Store};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: String,
    username: String,
    password: String
}

impl Store {
    pub fn create_user(&mut self, username: String, password: String) -> Result<String, Box<dyn std::error::Error>> {
        let id = Uuid::new_v4();
        let records = User {
            id: id.to_string(),
            username,
            password
        };
        diesel::insert_into(user::table)
            .values(&records)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;
        // print!("create user called")
        Ok(id.to_string())

    }
    pub fn get_user(&mut self, uname: String, pwd: String) -> Result<String, Box<dyn std::error::Error>> {
        // String::from("1")
        let results = user::table
            .filter(user::username.eq(uname))
            // .limit(5)
            .select(User::as_select())
            .first(&mut self.conn)?;
            // .expect("Error loading posts");

        if results.password != pwd {
            return Err(Box::new(diesel::result::Error::NotFound));
        }
        Ok(results.id)
    }
}
