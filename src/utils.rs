use std::io;
use std::io::{Read, Write};
use std::fs::File;


pub fn write_file(filename: &str, contents: &Vec<u8>) -> io::Result<()> {
    let mut file = try!(File::create(filename));
    Ok(try!(file.write_all(contents)))
}


pub fn read_file_to_string(filename: &str, buf: &mut String) -> io::Result<usize> {
    let mut file = try!(File::open(filename));
    let bytes = try!(file.read_to_string(buf));
    Ok(bytes)
}
