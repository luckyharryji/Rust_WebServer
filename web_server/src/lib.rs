use std::fs::File;
use std::path::Path;
use std::io::Result;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::sync::{Arc,Mutex};

// read the file from the htto request source
// for now. the type of the Reponse code is decided by the error returned by the File read.
pub fn get_file_content(path: &Path)->Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {  // rewrite with buffer read
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// write log into file
pub fn write_into_file(http_content: &str, log_file_with_lock: &Arc<Mutex<OpenOptions>>)->Result<()>{
	let mut log_file = log_file_with_lock.lock().unwrap();
	let mut f = try!(log_file.write(true).append(true).create(true).open("log.txt"));   // can not open and write????
	let content = http_content.to_owned();
	match f.write(content.as_bytes()){
		Ok(_) => Ok(()),
        Err(e) => {
        	return Err(e);
        },
	}
}

// return request information from status code
pub fn get_status_info<'a>(code:usize)->&'a str{
	match code{
		200 => "OK",
		400 => "Bad Request",
		403 => "Forbidden",
		404 => "Not Found",
		_ => "Code Error",  // never call here
	}
}
