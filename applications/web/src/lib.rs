use actix_web::{ App, HttpServer };
use actix_files as fs;
mod paths;

static PATH_FE: &'static str =  "./_FE/DemoApp/dist/";

pub async fn main() -> std::result::Result<(), std::io::Error> {
    actix_web::rt::System::new("web").block_on(async move {
        HttpServer::new(move || { 
            App::new()
            .service(paths::index)
            .service(paths::names_list)
            .service(paths::health_check)
            .service(fs::Files::new("/assets", format!("{}/assets", PATH_FE)))
            .service(fs::Files::new("/static", PATH_FE))
        }).bind("0.0.0.0:8080")?.run().await
    })
}