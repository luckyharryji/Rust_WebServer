use std::fs::File;
use std::path::Path;
use std::io::Result;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::sync::{Arc,Mutex};

// read the file from the http request source
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

#[cfg(test)]
mod lib_function_test {

	use super::{get_file_content, write_into_file, get_status_info};
	use std::fs::{File, remove_file};
	use std::io::prelude::*;
	use std::io::SeekFrom;
	use std::path::Path;

	#[test]
	fn status_info_test(){
		assert_eq!("OK", get_status_info(200));
		assert_eq!("Bad Request", get_status_info(400));
		assert_eq!("Forbidden", get_status_info(403));
		assert_eq!("Not Found", get_status_info(404));		
		assert_eq!("Code Error", get_status_info(100));
	}

	#[test]
	fn get_file_content_test(){
		let expected = "Hello world!\nThis is supposed to be read correctly\n".to_owned();

		let mut f = File::create("temp.txt").unwrap();

		f.write(expected.as_bytes()).unwrap();

		let under_test = get_file_content(Path::new("temp.txt")).unwrap();
		assert_eq!(expected, under_test);

		remove_file(Path::new("temp.txt")).unwrap();


	}

	#[test]
	fn write_file_file_test(){
		let line1 = "This is 1st test line\n";
		let line2 = "This is 2nd test line\n";

		let eof = {
			let mut f = File::open("log.txt").unwrap();
			f.seek(SeekFrom::End(0)).unwrap()
		};

		let expected = get_file_content(Path::new("log.txt")).unwrap();

		{
			write_into_file(&line1);

			let mut f = File::open("log.txt").unwrap();

			f.seek(SeekFrom::Start(eof));
			let mut s = String::new();
			f.read_to_string(&mut s).unwrap();
			assert_eq!(line1.to_owned(), s); 
			s.clear();
			drop(f);

			write_into_file(&line2);

			let mut f = File::open("log.txt").unwrap();

			f.seek(SeekFrom::Start(eof));
			f.read_to_string(&mut s).unwrap();
			assert_eq!(line1.to_owned() + line2, s); 
			drop(f);
		}

		
		let mut f = File::create("temp.txt").unwrap();
		f.write(expected.as_bytes()).unwrap();

	}
}