use rocket::{self};
use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};
use project;

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    let rocket = project::app();
    if is_running_on_lambda() {
        // Launch on AWS Lambda
        launch_rocket_on_lambda(rocket).await?;
    } else {
        // Launch local server
        let _ = rocket.launch().await?;
    }
    Ok(())
}