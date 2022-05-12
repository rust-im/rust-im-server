use rust_im_server;
use rocket::{self};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rust_im_server::rocket().launch().await?;
    Ok(())
}
