/*
 * @Author: Jean-François Ndi 
 * @Date: 2023-12-17 08:37:00 
 * @Last Modified by: Jean-François Ndi
 * @Last Modified time: 2023-12-17 09:08:21
 */
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main()
{
    HttpServer::new(
        || 
                App::new()
                    .route("/", web::get().to(HttpResponse::Ok)))
                    .workers(4);
}   
