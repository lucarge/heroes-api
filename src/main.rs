#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};

mod db;
mod hero;
mod schema;
use hero::{Hero};

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
  Json(json!({
    "status": "ok",
  }))
}

#[get("/")]
fn read() -> Json<Value> {
  Json(json!(["hero 1", "hero 2"]))
}

#[post("/", data="<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
  hero
}

#[put("/<id>", data="<hero>")]
fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
  hero
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}
