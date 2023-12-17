/*
 * @Author: Jean-François Ndi 
 * @Date: 2023-12-17 08:47:27 
 * @Last Modified by: Jean-François Ndi
 * @Last Modified time: 2023-12-17 09:01:25
 */
use actix_web::{web, App, HttpResponse, HttpServer};

//
// This function could be located in a different module.
//
fn scoped_config(cfg: &mut web::ServiceConfig)
{
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async {HttpResponse::Ok().body("test")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}

//
// This function could be located in a different module.
//
fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async {HttpResponse::Ok().body("app")}))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(
        ||
        {
            App::new()
                .configure(config)
                .service(web::scope("/api").configure(scoped_config))
                .route(
                    "/",
                    web::get().to(|| async {HttpResponse::Ok().body("/")}),
                )
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
