use std::io::prelude::*;
use std::net::TcpStream;

const REQUEST: &'static [u8] = b"GET /problem/s3cret HTTP/1.1\r\nHost: www.potw.quinnftw.com\r\n\r\n\0";

fn main() {
	let data = TcpStream::connect("www.potw.quinnftw.com:80")
		.and_then(|mut s| {
			try!(s.write(REQUEST));

			let mut buf = String::new();
			try!(s.read_to_string(&mut buf));
			Ok(buf)
		})
		.expect("Could not get data from stream.");

	println!("{}", data);
}
