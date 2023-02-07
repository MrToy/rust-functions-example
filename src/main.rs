use server;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = server::rocket().launch().await?;
    Ok(())
}