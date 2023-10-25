#[macro_use]
extern crate rocket;

use rocket::Request;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u32) -> String {
    format!("Hello, {}! You are {} years old.", name, age)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .register("/", catchers![not_found])
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, {} was not found.", req.uri())
}
