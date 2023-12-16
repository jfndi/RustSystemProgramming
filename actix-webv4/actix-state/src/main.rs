/*
 * @Author: Jean-François Ndi 
 * @Date: 2023-12-16 20:03:34 
 * @Last Modified by: Jean-François Ndi
 * @Last Modified time: 2023-12-16 20:17:16
 */
use actix_web::{get, web, App, HttpServer};

//
// This struct represents a state.
//
struct AppState
{
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String
{
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(
        ||
        {
            App::new()
                .app_data(web::Data::new(AppState
                {
                    app_name: String::from("Actix web"),
                }))
                .service(index)
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
