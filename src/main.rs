use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        eprintln!("Connection established! {:?}", &stream);
        handle_connection(stream);
    }
}

fn get_http_start_string() -> String {
    String::from("GET / HTTP/1.1\r\n")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    eprintln!("{}", String::from_utf8_lossy(&buffer[..]));
    let binding = get_http_start_string();
    let get = binding.as_bytes();
    let (status_line, filename) =
    if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::get_http_start_string;

    #[test]
    fn test_get_http_start_string() {
        assert_eq!(get_http_start_string(), String::from("GET / HTTP/1.1\r\n"));
    }
}
