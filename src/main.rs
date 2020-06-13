use std::path::Path;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web_codegen::{get, post};
use gumdrop::Options;
use thiserror::Error;

// --- Argument Parsing ---

#[derive(Error, Debug)]
pub enum CmdArgError {
    #[error("Path doesn't exist or is not a file: {0}")]
    BadPath(String),
    #[error("Not a valid X10 house code: {0}")]
    BadHouseCode(String),
    #[error("Not a valid X10 device number: {0}")]
    BadDeviceNumber(String),
}

/// Simple parser that a given path exists to catch typos, since anything fancier
/// would be much more work for little benefit in a long-running process.
/// (You need to expect to handle failures at the point of use anyway.)
fn parse_path(s: &str) -> Result<String, CmdArgError> {
    let string = s.to_owned();
    let path = Path::new(s);
    if path.exists() && !path.is_dir() { // is_file() would false out on /dev/ttyS0
        Ok(string)
    } else {
        Err(CmdArgError::BadPath(string))
    }
}

fn parse_house_code(s: &str) -> Result<String, CmdArgError> {
    let code = s.to_uppercase();
    if let Some(code_char) = code.chars().next() {
        if code.len() == 1 && code_char >= 'A' && code_char <= 'P' {
            return Ok(code)
        }
    }
    Err(CmdArgError::BadHouseCode(s.to_owned()))
}

fn parse_device_num(s: &str) -> Result<u8, CmdArgError> {
    if let Ok(num) = s.parse() {
        if num >= 1 && num <= 16 {
            return Ok(num)
        }
    }
    Err(CmdArgError::BadDeviceNumber(s.to_owned()))
}

#[derive(Clone, Debug, Options)]
struct CmdArgs {
    /// Show this help output
    help: bool,

    /// Path to the BottleRocket binary
    #[options(meta = "PATH", default = "/usr/bin/br", parse(try_from_str = "parse_path"))]
    br_path: String,

    /// Path to the X10 Firecracker's serial port
    #[options(meta = "PATH", default="/dev/ttyS0", parse(try_from_str = "parse_path"))]
    fc_path: String,

    /// X10 house code to pass to BottleRocket
    /// TODO: Validate that it's in the range A-P
    #[options(meta = "X", default="A", parse(try_from_str = "parse_house_code"))]
    house: String,

    /// X10 device number for "Turn off fan" button
    ///
    /// TODO: Validate that it's in the range 1-16
    #[options(meta = "N", default="1", parse(try_from_str = "parse_device_num"))]
    fan_id: u8,

    /// Port to listen for HTTP connections on
    #[options(meta = "N", default="8000")]
    port: u16,

    /// Number of times to repeat the X10 command to account for noisy/unreliable transmission
    #[options(meta = "N", default="4")]
    repeats: u8,
}

// --- HTTP Server ---

#[get("/")]
async fn control_panel(req: HttpRequest, data: web::Data<CmdArgs>) -> impl Responder {
    "TODO: Implement a control panel"
}

#[post("/fan_off")]
async fn fan_off(req: HttpRequest, data: web::Data<CmdArgs>) -> impl Responder {
    "TODO: Call bottlerocket to turn off fan"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let opts = CmdArgs::parse_args_default_or_exit();

    let port = opts.port;
    HttpServer::new(move || {
        App::new()
            .data(opts.clone())
            .service(control_panel)
            .service(fan_off)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
