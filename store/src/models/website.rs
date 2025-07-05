use crate::{schema::website, store::Store};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]

struct Website {
    id: String,
    url: String,
    user_id: String
}

impl Store {
    pub fn create_website(&mut self, url: String, user_id: String) -> Result<String, Box<dyn std::error::Error>>{
        // print!("create user called")
        let id = Uuid::new_v4();
        let records = Website {
            id: id.to_string(),
            url,
            user_id
        };
        diesel::insert_into(website::table)
            .values(&records)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)?;
        // print!("create user called")
        Ok(id.to_string())
    }
    pub fn get_website(&mut self, URL: String, user: String) -> Result<bool, Box<dyn std::error::Error>> {
        // String::from("1")
        let results = website::table
            .filter(website::url.eq(URL))
            // .limit(5)
            .select(Website::as_select())
            .first(&mut self.conn)?;
            // .expect("Error loading posts");

        if results.user_id != user {
            return Ok(false);
        }
        Ok(true)
    }
}
