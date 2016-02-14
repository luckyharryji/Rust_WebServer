
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

pub mod lib;
use lib::{get_file_content, write_into_file,get_status_info};


/**
Road Map:

Luo Fei, Nianzu, Xiangyu

Ramaining to be finish:
	- test case
	- log file
	- fully OOP design with url wrap
**/

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
				return Some(file_source.to_owned());
			},
			_ => return None,
		}
	}

	fn set_url(&mut self){
		match self.extract_url(){
			Some(url)=> {
				let mut file_addr = String::from("./");
				file_addr.push_str(&url);
				self.url = file_addr;
			},
			None => println!("url not exist"),
		}
	}


	fn process_url(&self)->Response{
		match self.url.ends_with("/"){
			true => return self.parse_dir(),
			false => return self.parse_file(),
		}
	}


	fn parse_dir(&self)->Response{
		let file_name = vec!["index.html", "index.shtml", "index.txt"];
		let origin_url = &self.url;
		for file in &file_name{
			let mut source_addr = origin_url.to_owned();
			source_addr.push_str(file);

			if let Ok(s) = get_file_content(&Path::new(&source_addr)){
				return self.form_response(200, Some(s));
			}
		}
		return self.form_response(404, None);
	}

	fn parse_file(&self)->Response{
		match get_file_content(&Path::new(&self.url)){
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


	fn record_log(&mut self){
		let mut strem_content = String::new();
		match self.stream.read_to_string(&mut strem_content){
			Ok(_) => {
				match write_into_file(strem_content){
					Err(_)=>println!("Log Record File"),
					Ok(_) => println!("Log Record Success"),
				}
			},
			Err(e) => println!("{}", e),
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
	// request.record_log();
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