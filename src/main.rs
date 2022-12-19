#![feature(proc_macro_hygiene, decl_macro)]

use std::{fs, io::Cursor};
use rocket::{Response, http::Status};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "alive"
}

#[post("/<task_name>", data = "<task_content>")]
fn receive_task(task_name: String, task_content: String) {
    match fs::write(&task_name, task_content) {
        Ok(_) => {
            println!("task received and saved: {}.", task_name)
        },
        Err(err) => {
            eprintln!("task failed to be saved: {}. : {}", task_name, err)
        }
    }
}

#[post("/<task_name>/run", data = "<task_params>")]
fn receive_task_params(task_name: String, task_params: String) -> Response<'static> {
    let params = task_params.split_whitespace();
    let mut command = std::process::Command::new("python3");
    command.arg(task_name).args(params);

    println!("About to run command: {:?}", command);
    let output = match command.output() {
        Ok(val) => val,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize()
    };

    Response::build().status(Status::Ok).sized_body(Cursor::new(output.stdout)).finalize()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/task", routes![receive_task, receive_task_params])
        .launch();
}
