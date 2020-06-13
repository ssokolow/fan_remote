use actix_web::{App, HttpRequest, HttpServer, Responder};
use actix_web_codegen::{get,post};

#[get("/")]
async fn control_panel(req: HttpRequest) -> impl Responder {
    format!("TODO: Implement a control panel")
}

#[post("/fan_off")]
async fn fan_off(req: HttpRequest) -> impl Responder {
    format!("TODO: Call bottlerocket to turn off fan")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(control_panel)
            .service(fan_off)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
