use std::io::Write;
use std::net::{TcpListener,TcpStream};
use std::thread;

fn handle_stream(mut stream:TcpStream){
	stream.write(b"test response\r\n").unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
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