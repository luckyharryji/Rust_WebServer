#[doc=" Web Server
	__author__ = 'Xiangyu Ji,  Nianzu Li'
	
	This program create a demo web server with Rust which can parse Get http request
	Host is in the local 127.0.0.1  port: 8000
	
	INPUT:
		The program will run to wait for the request to the host    
	
	OUTPUT:
		If the request url ends with a file name:
			- the file exists and is authorized to open:
				return response with code 200 and info 'OK' and the content of the file
			- the file dose not exist:
				return response with code 404 and info 'Not Found'
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
		- Program will automatically create file named 'log.txt' and write log inside if it does not exist
		- The file inside folder url request are ranked as 'index.html'>'index.shtml'>'index.txt'
		- Request only have GET type
		- Log are located in the host source, all log in one file named 'log.txt'
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

/**
Road Map:

Nianzu, Xiangyu

Ramaining to be finish:
	- test case
**/

fn handle_stream(stream:TcpStream,write_log_file: &Arc<Mutex<OpenOptions>>){
	let request_time = time::now().ctime().to_string();    // record time when request come
	let mut request = Request::new(stream);				   // parse the request, extract url and all requet info
	request.record_log(&request_time,write_log_file);					   // write request info into log

	let mut response = request.get_response();			   // create response structure from request information
	let reponse_code = response.write_response();		   // send back response to the client
	let response_time = time::now().ctime().to_string();   // record time when send out response
	response.record_log(&response_time, reponse_code,write_log_file);     // write request info into log
}


fn main() {
	// bing server to the localhost
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
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
			Err(e)=>{
				println!("{}", e);
			}
		}
    }
    // close server
    drop(listener);
}
