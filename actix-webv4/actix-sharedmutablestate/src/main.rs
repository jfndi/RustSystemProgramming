/*
 * @Author: Jean-François Ndi 
 * @Date: 2023-12-17 08:00:51 
 * @Last Modified by: Jean-François Ndi
 * @Last Modified time: 2023-12-17 08:21:30
 */
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter
{
    //
    // Mutex is necessary to mutate safely across threads.
    //
    counter:Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String
{
    //
    // Get the counter's MutexGuard.
    //
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    //
    // Response with count.
    //
    format!("Request number: {counter}")
}
#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    //
    // Note: web::data created _outside_ HttpServer::new closure.
    //
    let counter = web::Data::new(AppStateWithCounter
        {
            counter: Mutex::new(0),
        }
    );

    HttpServer::new(
        move ||
        {
            //
            // Move the counter into the closure.
            //
            App::new()
                //
                // Registers the created data.
                //
                .app_data(counter.clone())
                .route("/", web::get().to(index))
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
