extern crate tiny_http;
extern crate rustc_serialize;
extern crate rand;
extern crate unix_socket;

use std::thread;

use rustc_serialize::json;
use tiny_http::{Server, Response, Header, Method, Request, ResponseBox};

mod rpc;

#[derive(RustcDecodable, RustcEncodable)]
pub struct ResponseStruct  {
    value: String,
}

fn get_json_header() -> Header{
    Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap()
}

fn build_url_parts(request: &mut Request) -> Vec<String> {
    request.url().to_owned().split("/").map(|s| s.to_owned()).collect::<Vec<String>>()
}

fn serve(server: &Server) {
    for mut request in server.incoming_requests() {
        thread::spawn(move || {
            println!("received request! method: {:?}, url: {:?}",
                request.method(),
                request.url()
            );

            let res = match request.method() {
                &Method::Get => handle_get(&mut request),
                &Method::Post => handle_post(&mut request),
                _ => Response::empty(403).boxed(),
            };
            let _ = request.respond(res);
        });
    }
}

fn handle_get(mut request: &mut Request) -> ResponseBox {
    let url_parts = build_url_parts(request);
    
    if url_parts[1] == "api" && url_parts[2] == "analog" && url_parts[3] == "input" {
        match url_parts[4].parse::<i32>() {
            Ok(n) => get_analog_input(n),
            Err(_) => Response::empty(404).boxed(),
        }
    } else if url_parts[1] == "api" && url_parts[2] == "digital" && url_parts[3] == "input" {
        match url_parts[4].parse::<i32>() {
            Ok(n) => get_digital_input(n),
            Err(_) => Response::empty(404).boxed(),
        }
    } else {
        Response::empty(404).boxed()
    }
}

fn handle_post(mut request: &mut Request) -> ResponseBox {
    let url_parts = build_url_parts(request);
    
    if url_parts[1] == "api" && url_parts[2] == "analog" && url_parts[3] == "output" {

        match url_parts[4].parse::<i32>() {
            Ok(n) => post_analog_output(n, &mut request),
            Err(_) => Response::empty(404).boxed(),
        }
    } else if url_parts[1] == "api" && url_parts[2] == "digital" && url_parts[3] == "output" {
        match url_parts[4].parse::<i32>() {
            Ok(n) => post_digital_output(n, &mut request),
            Err(_) => Response::empty(404).boxed(),
        }
    } else {
        Response::empty(404).boxed()
    }
}

fn get_analog_input(id: i32)-> ResponseBox {
    let value = rpc::RPC::read_analog(id);
    let data = ResponseStruct{value: value.to_string()};
    Response::from_string(to_json(data)).with_header(get_json_header()).boxed()
}

fn get_digital_input(id: i32)-> ResponseBox {
    let value = rpc::RPC::read_digital(id);
    let data = ResponseStruct{value: value.to_string()};
    Response::from_string(to_json(data)).with_header(get_json_header()).boxed()
}

fn from_json(request: &mut Request) -> ResponseStruct {
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();

    json::decode(&content).unwrap()
}

fn to_json(content: ResponseStruct) -> String {
    json::encode(&content).unwrap()
}

fn post_analog_output(id: i32, mut request: &mut Request)-> ResponseBox {
    let body = from_json(&mut request);

    let value = rpc::RPC::write_analog(id, body.value.parse::<i32>().unwrap());
    let data = ResponseStruct{value: value.to_string()};

    Response::from_string(to_json(data)).with_header(get_json_header()).boxed()
}

fn post_digital_output(id: i32, mut request: &mut Request)-> ResponseBox {
    let body = from_json(&mut request);

    let value = rpc::RPC::write_digital(id, body.value.parse::<i32>().unwrap());
    let data = ResponseStruct{value: value.to_string()};

    Response::from_string(to_json(data)).with_header(get_json_header()).boxed()
}

fn main() {
    let server = Server::http("0.0.0.0:1337").unwrap();
    
    serve(&server);
}
