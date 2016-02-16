
use std::net::{TcpListener,TcpStream};
use std::thread;

pub mod lib;

mod response;

mod request;
use request::Request;

/**
Road Map:

Nianzu, Xiangyu

Ramaining to be finish:
	- test case
	- how to open and write rather than create
	- log: time and status code
**/

fn handle_stream(stream:TcpStream){
	let mut request = Request::new(stream);
	request.record_log();
	let mut send_response = request.get_response();
	send_response.write_response();
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
