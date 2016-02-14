use std::fs::File;
use std::path::Path;
use std::io::Result;
use std::io::prelude::*;

pub fn get_file_content(path: &Path)->Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn write_into_file(content: String)->Result<()>{
	let mut f = try!(File::open("log.txt"));
	match f.write(content.as_bytes()){
		Ok(_) => Ok(()),
        Err(e) => Err(e),
	}
}

pub fn get_status_info<'a>(code:usize)->&'a str{
	match code{
		200 => "OK",
		400 => "Bad Request",
		403 => "Forbidden",
		404 => "Not Found",
		_ => "Code Error",  // never call here
	}
}
