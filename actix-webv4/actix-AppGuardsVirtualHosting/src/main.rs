/*
 * @Author: Jean-François Ndi 
 * @Date: 2023-12-17 08:28:51 
 * @Last Modified by: Jean-François Ndi
 * @Last Modified time: 2023-12-17 08:43:29
 */
use actix_web::{web, App, guard, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(
        ||
        {
            App::new()
                .service(
                    web::scope("/")
                        .guard(guard::Host("www.rust-lang.org"))
                        .route("", web::to(|| async {HttpResponse::Ok().body("www")})),
                )
                .service(
                    web::scope("/")
                        .guard(guard::Host("users.rust-lang.org"))
                        .route("", web::to(|| async {HttpResponse::Ok().body("user")})),
                )
                .route("/", web::to(HttpResponse::Ok))
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
