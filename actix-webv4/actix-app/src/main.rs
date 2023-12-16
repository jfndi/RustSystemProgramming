/*
 * @Author: Jean-François Ndi 
 * @Date: 2023-12-16 19:36:25 
 * @Last Modified by: Jean-François Ndi
 * @Last Modified time: 2023-12-16 19:50:34
 */
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder
{
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> 
{
    HttpServer::new(
        ||
        {
            App::new()
            .service(
                //
                // Prefixes all resources and routes attached to it...
                //
                web::scope("/app")
                //
                // ...so this handles requests for `GET /app/index.html`
                //
                .route("/index.html", web::get().to(index)),
            )
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
