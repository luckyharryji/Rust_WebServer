use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;


pub mod lib;
use lib::{get_file_content};


struct Request{
	url: String,
	stream:TcpStream,
	fileType: Option<String>,
}


impl Request{
	fn new(stream:TcpStream)->Self{
		Request{
			url: "".to_owned(),
			stream:stream,
			fileType:None,
		}
	}

	fn extract_url(&self)->Option<String>{
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

	fn set_url(&mut self){
		match self.extract_url(){
			Some(url)=> self.url = url,
			None => println!("url not exist"),
		}
	}


	fn process_url(&self)->Response{
		let mut file_addr = String::from("./");
		file_addr.push_str(&self.url);
		match get_file_content(&Path::new(&file_addr)){
			Err(meg) => {
				match meg.kind(){
					ErrorKind::NotFound => self.form_response(404, None),
					ErrorKind::PermissionDenied => self.form_response(403, None),
					_ => self.form_response(400, None),
				}
			},
			Ok(s)=>self.form_response(200, Some(s)),
		}
	}


	fn form_response(&self, code:usize,content:Option<String>)->Response{
		println!("code is : {}", code);
		match content{
			Some(content) => {
				// println!("> {}", content)
				let length_of_content = content.len();
				return Response::new(code, Some(length_of_content), Some(content), Some("text".to_owned())); // xiangyu: rewrite to decide type
			},
			None => Response::new(code, None, None, Some("plain".to_owned())),
		}
	}

	fn get_response(&mut self)->Response{
		self.set_url();
		self.process_url()
	}
}


struct Response{
	statusCode: usize,
	contentLength: Option<usize>,
	content: Option<String>,
	fileType: Option<String>,
}

impl Response{
	fn new(code:usize, contentLength: Option<usize>, content:Option<String>, fileType:Option<String>)->Self{
		Response{
			statusCode: code,
			contentLength: contentLength,
			content:content,
			fileType: fileType,
		}
	}
}


fn handle_stream(stream:TcpStream){
	println!("haha {}", stream.local_addr().unwrap());
	println!("{}", stream.peer_addr().unwrap());
	let mut request = Request::new(stream);
	let test_response = request.get_response();
}

fn write_header(stream:&mut TcpStream){
	stream.write(b"test response\r\n").unwrap();
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