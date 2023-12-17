/*
 * @Author: Jean-François Ndi 
 * @Date: 2023-12-17 07:05:40 
 * @Last Modified by: Jean-François Ndi
 * @Last Modified time: 2023-12-17 07:17:00
 */
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(
        || App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
