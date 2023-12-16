/*
 * @Author: Jean-François Ndi 
 * @Date: 2023-12-16 15:15:09 
 * @Last Modified by: Jean-François Ndi
 * @Last Modified time: 2023-12-16 15:44:43
 */

/**
 * Request handlers use async functions that accept zero or more parameters.
 * These parameters can be extracted from a request and returns a type that can
 * be converted into an HttpResponse.
 */
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder
{
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder
{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder
{
    HttpResponse::Ok().body("Hey, there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(
        ||
        {
            App::new()
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
