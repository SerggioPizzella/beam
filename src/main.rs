#![feature(proc_macro_hygiene, decl_macro)]

use std::fs;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/task/<task_name>", data = "<task_content>")]
fn receive_task(task_name: String, task_content: String) {
    match fs::write(&task_name, task_content) {
        Ok(_) => {
            println!("task received and saved: {}.", task_name);
            return 
        },
        Err(err) => {
            eprintln!("task failed to be saved: {}. : {}", task_name, err)
        }
    }
}

#[get("/alive")]
fn heart_beat() -> &'static str {
    "alive"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, heart_beat, receive_task])
        .launch();
}
