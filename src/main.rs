use std::{fmt, io};
use std::path::Path;
use std::process::Command;

use actix_web::{error, rt, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::middleware::NormalizePath;
use gumdrop::Options;
use listenfd::ListenFd;
use thiserror::Error;

// --- Argument Parsing ---

/// Error enum for command-line argument validation errors
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
///
/// Gumdrop is an acceptable argument parser in this case, because the only paths it needs to
/// handle are so unlikely to contain non-UTF8 elements.
fn parse_path(s: &str) -> Result<String, CmdArgError> {
    let string = s.to_owned();
    let path = Path::new(s);
    if path.exists() && !path.is_dir() { // is_file() would false out on device nodes
        Ok(string)
    } else {
        Err(CmdArgError::BadPath(string))
    }
}

/// Parser/validator for X10 house codes
fn parse_house_code(s: &str) -> Result<String, CmdArgError> {
    let code = s.to_uppercase();
    if let Some(code_char) = code.chars().next() {
        if code.len() == 1 && code_char >= 'A' && code_char <= 'P' {
            return Ok(code)
        }
    }
    Err(CmdArgError::BadHouseCode(s.to_owned()))
}

/// Parser/validator for X10 device numbers
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

    /// Path to the PyCM19A binary
    #[options(meta = "PATH", default = "/usr/local/bin/pycm19a.py",
        parse(try_from_str = "parse_path"))]
    pycm19a_path: String,

    /// X10 house code to pass to PyCM19A
    #[options(meta = "X", default="A", parse(try_from_str = "parse_house_code"))]
    house: String,

    /// X10 device number for "Turn off fan" button
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

#[derive(Debug)]
enum PyCM19AErrorKind {
    SpawnFailure,
    ReturnedFailure,
}

/// Error type for failures to call PyCM19A as a subprocess
#[derive(Error, Debug)]
struct PyCM19AError {
    kind: PyCM19AErrorKind,
    source: io::Error,
}

impl fmt::Display for PyCM19AError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to ask for the fan to be turned off: {}",
        match self.kind {
            PyCM19AErrorKind::SpawnFailure => "Couldn't call PyCM19A",
            PyCM19AErrorKind::ReturnedFailure => "PyCM19A reported failure",
        })
    }
}
impl error::ResponseError for PyCM19AError {}

/// Barebones GET route to provide a "Turn Off Fan" button
async fn control_panel() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width" />

            <title>Remote Control</title>
            <style>
                body {
                    margin: 1em;
                    text-align: center;
                }
            </style>
        </head>
        <body>
            <form method="POST" action="/fan_off">
                <input type="submit" value="Turn Off Fan" />
            </form>
        </body>
    </html>

    "#)
}

/// Display a persistent notification so surprise fan_off commands can be diagnosed
#[cfg(feature = "notify-rust")]
fn notify(msg: &str) {
    if let Err(_) = notify_rust::Notification::new()
            .summary("Hall Fan Stopped")
            .body(msg)
            .icon("application-exit")
            .appname("fan_remote")
            .hint(notify_rust::Hint::Resident(true))
            .timeout(0)
            .show() {
        println!("{}", msg);
    }
}

/// If built without notify-rust, assume systemd socket activation and notify to stdout
#[cfg(not(feature = "notify-rust"))]
fn notify(msg: &str) {
    println!("{}", msg);
}

/// POST route to get called by the "Turn Off Fan" button
async fn fan_off(req: HttpRequest, data: web::Data<CmdArgs>) -> impl Responder {
    let msg = format!("A user at IP address {} requested that the fan be turned off.",
        req.peer_addr().map(|adr| adr.ip().to_string()).unwrap_or("<unknown>".to_owned()));
    notify(&msg);

    // Shell out to PyCM19A in as secure a manner as possible to control fan via X10
    // (Trusts the CLI argument parser to have validated the non-constant parts)
    for _ in 0..data.repeats {
        Command::new("python3")
            .arg(&data.pycm19a_path)
            .arg(format!("-{}{}", data.house, data.fan_id))
            .spawn()
            .map_err(|e| PyCM19AError { kind: PyCM19AErrorKind::SpawnFailure, source: e })?
            .wait()
            .map_err(|e| PyCM19AError { kind: PyCM19AErrorKind::ReturnedFailure, source: e })?;
    }
    Ok::<&str, PyCM19AError>("X10 doesn't support confirming, but the fan should be off now.")
}

fn main() -> std::io::Result<()> {
    let opts = CmdArgs::parse_args_default_or_exit();

    let port = opts.port;
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .app_data(web::Data::new(opts.clone()))
            .route("/", web::get().to(control_panel))
            .route("/fan_off", web::post().to(fan_off))
    });

    // Use listenfd to support systemd socket activation for tighter sandboxing
    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(("0.0.0.0", port))?
    };

    rt::System::new().block_on(server.run())
}
