#[doc=" Web Server
	__author__ = 'Xiangyu Ji,  Nianzu Li'
	
	This program create a demo web server with Rust which can parse Get http request
	Host is in the local 127.0.0.1  port: 8080

	Reference:
		The way to write test for this server is finished by the discussion with Jiaju Ni
	
	INPUT:
		The program will run to wait for the request to the host    
	
	OUTPUT:
		If the request url ends with a file name:
			- the file exists and is authorized to open:
				return response with code 200 and info 'OK' and the content of the file
			- the file dose not exist:
				return response with code 404 and return 404 page html.
			- the file can are not authorized to open:
				return response with code 403 and info 'Forbidden'
			- caused error during this process:
				return response with code 400 and info 'Bad Request'

		If the request url ends with a location of a folder:
			- there are file inside the folder names: 'index.html', 'index.shtml', 'index.txt' and the file is authorized to open:
				return response with code 200 and info 'OK' and the content of the file
			- no such 3 file or the file is not authorized to open:
				return response with code 400 and info 'Bad Request'

	    If the request meets network error and can not be caought:
	    	- print error type
	    If the request lost header or not in the right format:
	    	- print 'Request Error'

		If log can be record correctly:
			- print 'Log recorded'
	    If error during the log recording process:
	    	- print 'Failed to record log'


	Assumptions:
		- If requested file does not exists, return a 404.html page in the source folder.
		- The request file path is in the Linux File path format, not support for windows
		- The HTTP request must have a blank line in the end
		- Log will record all the browser info if request is sent by the browser
		- Program will automatically create file named 'log.txt' and write log inside if it does not exist
		- The file inside folder url request are ranked as 'index.html'>'index.shtml'>'index.txt'
		- Request only support GET type
		- Log file are located in the host source, all log in one file named 'log.txt'
		- Lock on the log.txt, one request/reponse can add at a time
		- The first line (request line) of request contains all the information we need
		- Request header ends up in a blank line
"]


use std::net::{TcpListener,TcpStream};
use std::thread;
use std::sync::{Arc,Mutex};
use std::fs::OpenOptions;

extern crate time;  // import for record time for log

pub mod lib;

mod response;

mod request;
use request::Request;


fn main() {
	initial_bind_server(8080);
}


fn handle_stream(stream:TcpStream,write_log_file: &Arc<Mutex<OpenOptions>>){
	let request_time = time::now().ctime().to_string();    // record time when request come
	let mut request = Request::new(stream);				   // parse the request, extract url and all requet info
	request.record_log(&request_time,write_log_file);					   // write request info into log

	let mut response = request.get_response();			   // create response structure from request information
	let reponse_code = response.write_response();		   // send back response to the client
	let response_time = time::now().ctime().to_string();   // record time when send out response
	response.record_log(&response_time, reponse_code,write_log_file);     // write request info into log
}


fn initial_bind_server(port:usize){
	// bing server to the localhost
	let bind_addr:&str = &("127.0.0.1:".to_owned()+&port.to_string());
    let listener = TcpListener::bind(bind_addr).unwrap();
    println!("Server Started");

    let file_for_log = Arc::new(Mutex::new(OpenOptions::new()));
    for stream in listener.incoming() {
    	let log_file_for_write = file_for_log.clone();
		match stream{
			Ok(stream)=>{  				
				thread::spawn(move || {  // spawn a thread for each request 
					handle_stream(stream,&log_file_for_write);
				});
			},
			Err(_)=>{
				println!("Reques Stream Error");
			}
		}
    }
    // close server
    drop(listener);
}


/** test framework**/
#[cfg(test)]
mod http_stream_test {
	use super::{initial_bind_server};
	use std::net::TcpStream;
	use std::thread;
	use std::io::prelude::*;
	use std::time::Duration;

    fn build_connect(port:usize, request_header: String) -> String {
        let mut actual_response = String::new();
        let sock_addr:&str = &("127.0.0.1:".to_owned()+&port.to_string());
        let mut stream = match TcpStream::connect(sock_addr) {
            Ok(s)   => s,
            Err(_)  => return actual_response,
        };
        if let Err(_) = stream.write(request_header.as_bytes()) {
            return actual_response;
        }
        if let Err(_) = stream.read_to_string(&mut actual_response) {
            return actual_response;
        }
        return actual_response;
    }

	#[test]
	fn test_success_get_file(){
		thread::spawn(move || {
			initial_bind_server(8000);
        });
		thread::sleep(Duration::new(1, 0));
		let http_request_info = "GET /test.html HTTP/1.1\n\r\n".to_owned();
		let actual_response = build_connect(8000, http_request_info);
		let response_info = "HTTP/1.0 200 OK\r\nXiangyu and Nianzu: Rust-Server/0.1\r\nContent-type: text/html\r\nContent-length: 101\r\n\r\n<!DOCTYPE html>\n<html>\n<body>\n\n<h1>My First Heading</h1>\n\n<p>My first paragraph.</p>\n\n</body>\n</html>\r\n".to_owned();
		assert_eq!(actual_response, response_info);
	}

	#[test]
	fn test_success_get_html(){
		thread::spawn(move || {
			initial_bind_server(8002);
        });
		thread::sleep(Duration::new(1, 0));
		let http_request_info = "GET /test.txt HTTP/1.1\n\r\n".to_owned();
		let actual_response = build_connect(8002, http_request_info);
		let response_info = "HTTP/1.0 200 OK\r\nXiangyu and Nianzu: Rust-Server/0.1\r\nContent-type: text/plain\r\nContent-length: 13\r\n\r\na simple test\r\n".to_owned();
		assert_eq!(actual_response, response_info);
	}

	#[test]
	fn test_not_found_request(){
		thread::spawn(move || {
			initial_bind_server(8001);
        });
		thread::sleep(Duration::new(1, 0));
		let http_request_info = "GET /not_find.txt HTTP/1.1\n\r\n".to_owned();
		let actual_response = build_connect(8001, http_request_info);
		let response_info = "HTTP/1.0 404 Not Found\r\nXiangyu and Nianzu: Rust-Server/0.1\r\nContent-type: text/html\r\nContent-length: 71\r\n\r\n<!DOCTYPE html>\n<html>\n<body>\n\n<h1>Page Not Found</h1>\n\n</body>\n</html>\r\n".to_owned();
		assert_eq!(actual_response, response_info);		
	}


	#[test]
	fn test_get_index_html_from_folder(){
		thread::spawn(move || {
			initial_bind_server(8003);
        });
		thread::sleep(Duration::new(1, 0));
		let http_request_info = "GET /test_folder/ HTTP/1.1\n\r\n".to_owned();
		let actual_response = build_connect(8003, http_request_info);
		let response_info = "HTTP/1.0 200 OK\r\nXiangyu and Nianzu: Rust-Server/0.1\r\nContent-type: text/html\r\nContent-length: 103\r\n\r\n<!DOCTYPE html>\n<html>\n<body>\n\n<h1>Html inside folder</h1>\n\n<p>My first paragraph.</p>\n\n</body>\n</html>\r\n".to_owned();
		assert_eq!(actual_response, response_info);		
	}
}
