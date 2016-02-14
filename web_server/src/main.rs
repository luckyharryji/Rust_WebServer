extern crate regex;

use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use regex::Regex;

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

	fn extract_url(&self)->Option<String>{
		let mut reader_method = BufReader::new(&self.stream).lines();
		match reader_method.next(){
			Some(Ok(line)) =>{
				let request_info = line.to_owned();
				let http_info:Vec<&str> = request_info.split_whitespace().collect();
				let file_source = http_info[1];

				println!("Try regex {} ends with {}", &file_source, &file_source.ends_with(".txt"));
				
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
		// Only have conten-type when get a file with code 200
		let response_file_type = match code{
			200=>{
				match self.url.ends_with(".html"){
					true => Some("html".to_owned()),
					false => Some("plain".to_owned()),
				}
			},
			_ => None,
		};

		match content{
			Some(content) => {
				let length_of_content = content.len();
				return Response::new(code, Some(length_of_content), Some(content), response_file_type, &self.stream); // xiangyu: rewrite to decide type
			},
			None => Response::new(code, None, None, response_file_type, &self.stream),
		}
	}

	fn get_response(&mut self)->Response{
		self.set_url();
		self.process_url()
	}
}


struct Response<'a>{
	status_code: usize,
	content_length: Option<usize>,
	content: Option<String>,
	file_type: Option<String>,
	stream: &'a TcpStream,
}

impl <'a>Response<'a>{
	fn new(code:usize, content_length: Option<usize>, content:Option<String>, file_type:Option<String>,stream:&'a TcpStream)->Self{
		Response{
			status_code: code,
			content_length: content_length,
			content:content,
			file_type: file_type,
			stream: stream,
		}
	}


	fn write_response(&mut self){
		let status_info = get_status_info(self.status_code).to_owned();
		let header = format!("HTTP/1.0 {} {}\r\n", self.status_code, status_info);
		let server_name = "Xiangyu and Nianzu: Rust-Server/0.1\r\n";

		let response_type_info = match self.file_type{
			Some(ref file_type)=>format!("Content-type: text/{}\r\n",file_type.to_owned()),
			None => "".to_owned(),
		};

		let response_content_length_info = match self.content_length{
			Some(length) => format!("Content-length: {}\r\n",length),
			None => "".to_owned(),
		};

		let response_content = match self.content{
			Some(ref content) => format!("\r\n{}\r\n",content),
			None => "".to_owned(),
		};

		let result = header+server_name+&response_type_info+&response_content_length_info+&response_content;

		println!("{}",result);
		self.write_to_stream(&result);
	}

	fn write_to_stream(&mut self, content:&str){
		let response_write_content = content.to_owned();
		self.stream.write(response_write_content.as_bytes()).unwrap();
	}
}


fn handle_stream(stream:TcpStream){
	let mut request = Request::new(stream);
	let mut send_response = request.get_response();
	send_response.write_response();
}


fn get_status_info<'a>(code:usize)->&'a str{
	match code{
		200 => "OK",
		400 => "Bad Request",
		403 => "Forbidden",
		404 => "Not Found",
		_ => "Code Error",  // never call here
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