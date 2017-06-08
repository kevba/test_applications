use unix_socket::UnixStream;
use std::collections::HashMap;
use rustc_serialize::json;
use std::io::Read;
use std::io::Write;

#[derive(RustcDecodable, RustcEncodable)]
pub struct RPCRequestStruct  {
    jsonrpc: String,
    method: String,
    params: HashMap<String, i32>,
    id: i32,
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct RPCResponseStruct  {
    result: f32,
    jsonrpc: String,
    id: i32,
}

pub struct RPC;

impl RPCRequestStruct {
    fn new(method: String, params: HashMap<String, i32>) -> RPCRequestStruct {
        RPCRequestStruct {
            jsonrpc: "2.0".to_string(),
            method: method,
            params: params,
            id: 11
        }
    }
}

impl RPC {
    pub fn from_json(buffer: &mut String) -> RPCResponseStruct {
        println!("{:?}", buffer);
        match json::decode::<RPCResponseStruct>(&buffer) {
            Ok(value) => println!("{:?}", value),
            Err(err) => println!("{}", err),
        };
        json::decode(buffer).unwrap()
    }

    pub fn send_message(message: RPCRequestStruct) -> RPCResponseStruct {
        let mut stream = UnixStream::connect("/tmp/iris").unwrap();

        // ignore the Result
        let _ = stream.write(json::encode(&message).unwrap().as_bytes());

        let mut buffer = String::new();
        let _ = stream.read_to_string(&mut buffer);
        
        RPC::from_json(&mut buffer)
    }

    pub fn read_analog(id: i32) -> i32 {
        let mut params = HashMap::new();
        params.insert("port_number".to_string(), id);
        let msg = RPCRequestStruct::new("read_analog".to_string(), params);
        let resp = RPC::send_message(msg);
        resp.result as i32
    }
    
    pub fn read_digital(id: i32) -> i32 {
        let mut params = HashMap::new();
        params.insert("port_number".to_string(), id);
        let msg = RPCRequestStruct::new("read_digital".to_string(), params);
        let resp = RPC::send_message(msg);
        resp.result as i32
    }

    pub fn write_analog(id: i32, value: i32) -> i32 {
        let mut params = HashMap::new();
        params.insert("port_number".to_string(), id);
        params.insert("value".to_string(), value);
        let msg = RPCRequestStruct::new("write_analog".to_string(), params);
        let resp = RPC::send_message(msg);
        resp.result as i32
    }

    pub fn write_digital(id: i32, value: i32) -> i32 {
        let mut params = HashMap::new();
        params.insert("port_number".to_string(), id);
        params.insert("value".to_string(), value);
        let msg = RPCRequestStruct::new("write_digital".to_string(), params);
        let resp = RPC::send_message(msg);
        resp.result as i32
    }

}
