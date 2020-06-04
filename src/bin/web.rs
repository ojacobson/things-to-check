use actix_web::{App, HttpServer};
use std::io;
use thiserror::Error;

use things_to_check::twelve;
use things_to_check::view;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to determine port number: {0}")]
    PortError(#[from] twelve::Error),
    #[error("Unable to initialize web view: {0}")]
    ViewError(#[from] view::Error),
    #[error("Unexpected IO error: {0}")]
    IOError(#[from] io::Error),
}

type Result = std::result::Result<(), Error>;

#[actix_rt::main]
async fn main() -> Result {
    let port = twelve::port(3000)?;

    let service = view::make_service()?;

    let app_factory = move ||
        App::new()
            .configure(|cfg| service(cfg));

    HttpServer::new(app_factory)
        .bind(port)?
        .run()
        .await?;
    
    Ok(())
}
