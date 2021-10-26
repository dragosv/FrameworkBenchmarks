#![feature(proc_macro_hygiene, decl_macro)]

extern crate rand;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate lazy_static;

mod models;
mod world;
mod fortune;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::config::{Config};
use rocket::response::content;
use rocket::response::content::Json;
use rocket::{Build, Request, Rocket};
use std::sync::Mutex;
use sea_orm_rocket::{Connection, Database};
use yarte::Template;

struct RandomArray {
    pointer: usize,
    size: i32,
    data: Vec<i32>,
}

impl RandomArray {
    fn new(size: i32) -> Self {
        let mut data: Vec<i32> = (1..=size).collect();
        let mut rng = thread_rng();
        data.shuffle(&mut rng);

        RandomArray {
            pointer: 0,
            size,
            data,
        }
    }

    fn next(&mut self) -> i32 {
        if self.pointer >= self.size as usize {
            self.pointer = 1;
        } else {
            self.pointer += 1;
        }
        self.data[self.pointer - 1]
    }
}

lazy_static! {
    static ref RANDOM_ARRAY: Mutex<RandomArray> = Mutex::new(RandomArray::new(10000));
}
fn random_number() -> i32 {
    RANDOM_ARRAY
        .lock()
        .expect("Failed to lock RANDOM_ARRAY")
        .next()
}

#[get("/plaintext")]
async fn plaintext() -> &'static str {
    "Hello, world::Model!"
}

#[get("/json")]
async fn json() -> Json<models::Message> {
    let message = models::Message {
        message: "Hello, world::Model!",
    };
    Json(message)
}

#[get("/db")]
async fn db(conn: Connection<'_, Db>) -> Json<world::Model> {
    let db = conn.into_inner();

    let result : world::Model = world::Model::Model::find_by_id(random_number()).one(db).await.expect("error loading world::Model");

    Json(result)
}

#[get("/queries")]
async fn queries_empty(conn: Connection<'_, Db>) -> Json<Vec<world::Model>> {
    queries(conn, 1)
}

#[get("/queries?<q>")]
async fn queries(conn: Connection<'_, Db>, q: u16) -> Json<Vec<world::Model>> {
    let db = conn.into_inner();

    let q = if q == 0 {
        1
    } else if q > 500 {
        500
    } else {
        q
    };

    let mut results = Vec::with_capacity(q as usize);

    for _ in 0..q {
        let query_id = random_number();

        let result = world::Model::find_by_id(query_id).one(db).await?;

        // let result = world::Model
        //     .filter(id.eq(query_id))
        //     .first::<world::Model>(&*conn)
        //     .unwrap_or_else(|_| panic!("error loading world::Model, id={}", query_id));
        results.push(result);
    }

    Json(results)
}

#[derive(Template)]
#[template(path = "fortunes.html.hbs")]
pub struct FortunesTemplate<'a> {
    pub fortunes: &'a Vec<fortune::Model>,
}

#[get("/fortunes")]
async fn fortunes(conn: Connection<'_, Db>) -> content::Html<String> {
    let db = conn.into_inner();

    let mut fortunes: Vec<fortune::Model> = fortune::Model::find().all(db).await?;
        // fortune
        // .load::<models::fortune::Model>(&*conn)
        // .expect("error loading fortunes");

    fortunes.push(fortune::Model {
        id: 0,
        message: "Additional fortune added at request time.".to_string(),
    });

    fortunes.sort_by(|a, b| a.message.cmp(&b.message));

    content::Html(
        FortunesTemplate {
            fortunes: &fortunes,
        }
        .call()
        .expect("error rendering template"),
    )
}

#[get("/updates")]
async fn updates_empty(conn: Connection<'_, Db>) -> Json<Vec<world::Model>> {
    updates(conn, 1)
}

#[get("/updates?<q>")]
async fn updates(conn: Connection<'_, Db>, q: u16) -> Json<Vec<world::Model>> {
    let db = conn.into_inner();

    let q = if q == 0 {
        1
    } else if q > 500 {
        500
    } else {
        q
    };

    let mut results = Vec::with_capacity(q as usize);

    for _ in 0..q {
        let query_id = random_number();
        let mut result = world::Model::find_by_id(query_id).one(db).await?;

            // world::Model
            // .filter(id.eq(query_id))
            // .first::<world::Model>(&*conn)
            // .unwrap_or_else(|_| panic!("error loading world::Model, id={}", query_id));
        result.randomNumber = random_number();
        results.push(result);
    }

    // let _ = conn.transaction::<(), Error, _>(|| {
    //     for w in &results {
    //         let _ = diesel::update(world::Model)
    //             .filter(id.eq(w.id))
    //             .set(randomnumber.eq(w.randomNumber))
    //             .execute(&*conn);
    //     }
    //     Ok(())
    // });

    Json(results)
}

fn main() {
    let mut config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(8000)
        .log_level(LoggingLevel::Off)
        .workers((num_cpus::get() * 16) as u16)
        .keep_alive(0)
        .expect("failed to generate config");
    config
        .set_secret_key("dY+Rj2ybjGxKetLawKGSWi6EzESKejvENbQ3stffZg0=")
        .expect("failed to set secret");

    rocket::custom(config)
        .mount(
            "/",
            routes![
                json,
                plaintext,
                db,
                queries,
                queries_empty,
                fortunes,
                updates,
                updates_empty,
            ],
        )
        //.manage(db::init_pool())
        .launch();
}
