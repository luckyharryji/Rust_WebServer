
use std::net::{TcpListener,TcpStream};
use std::thread;

extern crate time;

pub mod lib;

mod response;

mod request;
use request::Request;

/**
Road Map:

Nianzu, Xiangyu

Ramaining to be finish:
	- test case
	- rewrite file read buffer
**/

fn handle_stream(stream:TcpStream){
	let request_time = time::now().ctime().to_string();
	let mut request = Request::new(stream);
	request.record_log(&request_time);

	let mut response = request.get_response();
	let reponse_code = response.write_response();
	let response_time = time::now().ctime().to_string();
	response.record_log(&response_time, reponse_code);
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("listening started, ready to accept");

    for stream in listener.incoming() {
		match stream{
			Ok(stream)=>{
				thread::spawn(move || {
					handle_stream(stream);
				});
			},
			Err(e)=>{
				println!("{}", e);
			}
		}
    }
    drop(listener);
}
