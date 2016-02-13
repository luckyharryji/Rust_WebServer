// use std::io::{Write,Read};
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::Result;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


fn handle_stream(mut stream:TcpStream){
	println!("haha {}", stream.local_addr().unwrap());
	println!("{}", stream.peer_addr().unwrap());
	stream.write(b"test response\r\n").unwrap();

	let mut reader_method = BufReader::new(stream).lines();
	if let Some(Ok(line)) = reader_method.next(){
		let request_info = line.to_owned();
		let http_info:Vec<&str> = request_info.split_whitespace().collect();
		let file_source = http_info[1];
		process_url(file_source);
	}
}

fn get_file_content(path: &Path)->Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn process_url(file_source:&str){
	println!("{}", file_source);
	let mut file_addr = String::from("./");
	file_addr.push_str(file_source);
	match get_file_content(&Path::new(&file_addr)){
		Err(meg) => println!("!{:?}", meg.kind()),
		Ok(s)=>println!("> {}", s),
	}
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