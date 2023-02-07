use project;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = project::app().launch().await?;
    Ok(())
}