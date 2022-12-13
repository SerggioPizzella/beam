#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn heart_beat() -> &'static str {
    "alive"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/alive", routes![heart_beat])
        .launch();
}
