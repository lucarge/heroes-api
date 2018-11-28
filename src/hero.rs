#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::heroes;

#[table_name = "heroes"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: Hero, connection: &MysqlConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero.");
        
        heroes::table.order(heroes::id.desc()).first(connection).unwrap()
    }
}
