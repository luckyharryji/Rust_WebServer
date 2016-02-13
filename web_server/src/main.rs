// use std::io::{Write,Read};
// extern crate regex;

use std::net::{TcpListener,TcpStream};
use std::thread;
// use std::io::Result;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::ErrorKind;
// use std::fs::File;
use std::path::Path;
// use regex::Regex;


pub mod lib;
use lib::{get_file_content};


struct Request{
	url: String,
	stream:TcpStream,
}


impl Request{
	fn new(stream:TcpStream)->Self{
		Request{
			url: "".to_owned(),
			stream:stream,
		}
	}

	fn extract_url(&mut self)->Option<String>{
		let mut reader_method = BufReader::new(&self.stream).lines();
		match reader_method.next(){
			Some(Ok(line)) =>{
				let request_info = line.to_owned();
				let http_info:Vec<&str> = request_info.split_whitespace().collect();
				let file_source = http_info[1];
				return Some(file_source.to_owned());
			},
			_ => return None,
		}
	}
}


fn handle_stream(mut stream:TcpStream){
	println!("haha {}", stream.local_addr().unwrap());
	println!("{}", stream.peer_addr().unwrap());
	let mut request = Request::new(stream);
	match request.extract_url(){
		Some(url) => println!("get url in this way {}", url),
		_ => println!("Error here"),
	}
}

fn write_header(stream:&mut TcpStream){
	stream.write(b"test response\r\n").unwrap();
}

fn process_url(file_source:&str){
	println!("{}", file_source);
	let mut file_addr = String::from("./");
	file_addr.push_str(file_source);
	match get_file_content(&Path::new(&file_addr)){
		Err(meg) => {
			match meg.kind(){
				// rewrite to return code here
				ErrorKind::NotFound => println!("can not find file"), //404
				ErrorKind::PermissionDenied => println!("Permission denied"), //403
				_ => println!("!{:?}", meg.kind()), //400
			}
		},
		Ok(s)=>println!("> {}", s),  //200
	}
}


fn status_info<'a>(code:usize)->&'a str{
	match code{
		200 => "OK",
		400 => "Bad Request",
		403 => "Forbidden",
		404 => "Not Found",
		_ => "code Error",
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