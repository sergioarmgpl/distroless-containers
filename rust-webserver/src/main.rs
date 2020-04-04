extern crate time;

use std::io::Write;
use std::net::TcpListener;

fn main() {
    // Bind allows us to create a connection on the port
    // and gets it ready to accept connections.
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    // The listener's accept method waits or 'blocks' until
    // we have a connection and then returns a new TcpStream
    // that we can read and write data to.
    let mut stream = listener.accept().unwrap().0;
    let message    = "Hello, World! Operating Systems";
    let format     = "%a, %d %b %Y %T GMT";
    let time       = time::now_utc();
    let response   = format!("HTTP/1.1 200 OK\r\n\
                              Date: {}\r\n\
                              Content-Type: text/html; charset=utf-8\r\n\
                              Content-Length: {}\r\n\
                              \r\n\
                              {}",
                              time::strftime(format, &time).unwrap(),
                              message.len(),
                              message);
    let _          = stream.write(response.as_bytes());
}
