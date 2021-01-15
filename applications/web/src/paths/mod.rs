use actix_web::{ web, post, get, HttpResponse, Responder, Result };
use std::path::PathBuf;
use actix_files as fs;

#[post("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
pub async fn index() -> Result<actix_files::NamedFile> {
    let path: PathBuf = format!("{}/index.html", super::PATH_FE).parse().unwrap();
    Ok(fs::NamedFile::open(path)?)
}

#[get("/api/names")]
pub async fn names_list() -> Result<web::Json<NamesResponse>> {
    let mut outputs: Vec<String> = vec![];
    // Return {names ["Jeff", "Erich", "Jason", "Chris", "Garrett", "Raj"]}
    Ok(web::Json(NamesResponse { names: outputs } ))
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NamesResponse {
    pub names: Vec<String>
}