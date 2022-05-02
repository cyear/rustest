use actix_web::{get, App, HttpResponse, HttpServer, Responder};
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("welcome！")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 注释
    let ip = "0.0.0.0:8000";
    println!("start http: {} ...", ip);
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(ip)?
    .run()
    .await
}
