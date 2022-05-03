/* Copyright (C) 2022 1274210585@qq.com
 *   This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *   This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
 *   You should have received a copy of the GNU General Public License
 *   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
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
