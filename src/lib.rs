#[macro_use]
extern crate rocket;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn app() -> _ {        
    rocket::build()
        .mount("/api", routes![index])
}
