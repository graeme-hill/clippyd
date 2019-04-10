#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::io::Read;
use rocket::config::{Config, Environment};

#[post("/set_clipboard_content", data = "<content>")]
fn set_clipboard_content(content: rocket::Data) {
    let mut req_stream = content.open();
    let mut buffer = String::new();
    match req_stream.read_to_string(&mut buffer) {
        Err(_) => {
            // todo: log an error
        },
        Ok(_) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(buffer).unwrap();
        }
    }

}

fn main() {
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(36912)
        .finalize();

    match config {
        Ok(c) => {
            rocket::custom(c).mount("/", routes![set_clipboard_content]).launch();
        },
        Err(_) => {
            // todo: log an error
        }
    };
}
