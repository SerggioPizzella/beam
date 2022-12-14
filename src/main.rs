#![feature(proc_macro_hygiene, decl_macro)]

use std::{fs, io::Cursor};

use rocket::{Response, http::Status};

#[macro_use]
extern crate rocket;

#[get("/alive")]
fn heart_beat() -> &'static str {
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
    let mut params = task_params.split_whitespace();

    let start_param = match params.next() {
        Some(val) => val,
        None => return Response::build().status(Status::BadRequest).finalize()
    };

    let stop_param = match params.next() {
        Some(val) => val,
        None => return Response::build().status(Status::BadRequest).finalize()
    };

    let mut command = std::process::Command::new("python3");
    command.arg(task_name).arg(start_param).arg(stop_param);

    println!("About to run command: {:?}", command);
    let output = match command.output() {
        Ok(val) => val,
        Err(_) => return Response::build().status(Status::InternalServerError).finalize()
    };

    Response::build().status(Status::Ok).sized_body(Cursor::new(output.stdout)).finalize()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![heart_beat])
        .mount("/task", routes![receive_task, receive_task_params])
        .launch();
}
