use std::io::{Write,Read};
use std::net::{TcpListener,TcpStream};
use std::thread;

fn handle_stream(mut stream:TcpStream){
	println!("haha {}", stream.local_addr().unwrap());
	println!("{}", stream.peer_addr().unwrap());
	stream.write(b"test response\r\n").unwrap();
	// let mut x = Vec::<u8>:new();
	let mut buf = String::new();
	stream.read_to_string(&mut buf);
	println!("{}", buf);
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
    // drop(listener);
}

// use std::net::{Shutdown, TcpListener};
// use std::thread;
// use std::io::Write;
 
// const RESPONSE: &'static [u8] = b"HTTP/1.1 200 OK\r
// Content-Type: text/html; charset=UTF-8\r\n\r
// <!DOCTYPE html><html><head><title>Bye-bye baby bye-bye</title>
// <style>body { background-color: #111 }
// h1 { font-size:4cm; text-align: center; color: black;
// text-shadow: 0 0 2mm red}</style></head>
// <body><h1>Goodbye, world!</h1></body></html>\r";
 
 
// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
 
//     for stream in listener.incoming() {
//         thread::spawn(move || {
//             let mut stream = stream.unwrap();
//             match stream.write(RESPONSE) {
//                 Ok(_) => println!("Response sent!"),
//                 Err(e) => println!("Failed sending response: {}!", e),
//             }
//             stream.shutdown(Shutdown::Write).unwrap();
//         });
//     }
// }