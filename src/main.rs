extern crate ascii;
extern crate tiny_http;

use std::path::Path;
use std::{fs,env};
use ascii::AsciiString;


fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e
    };

    match extension.to_str().unwrap() {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8"
    }
}

fn main() {
    // init webserver
    let server = tiny_http::Server::http("0.0.0.0:8080").unwrap();
    let port = server.server_addr().port();
    
    // grab filename or panic if none is given
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        panic!("no file provided!")
    }

    // attempt to open a file descriptor
    let file_name = args[1].to_string();
    let file_path = std::path::Path::new(&file_name);
    let file = fs::File::open(&file_path);

    // check if there's an error with the filedescriptor
    if file.is_err() {
        panic!("cannot read the file!");
    }

    // this is a little confusing but this convers the file var from Result<T> to a file
    let file = file.unwrap();

    println!("Now serving file {} on port {}", file_name, port);
    loop {

        let file_clone = file.try_clone().unwrap();
        
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break
        };

        println!("{:?}", rq);

    
        let response = tiny_http::Response::from_file(file_clone);

        let response = response.with_header(
            tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_ascii(get_content_type(&file_path)).unwrap(),
            }
        );

        let _ = rq.respond(response);
    }
}