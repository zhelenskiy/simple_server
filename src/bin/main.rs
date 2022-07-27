use std::{fs, io};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use simple_server::ThreadPool;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    let pool = ThreadPool::new(64);

    for stream in listener.incoming() {
        let stream = stream?;
        pool.execute(|| {
            handle_stream::<1024>(stream).unwrap_or_else(|x| println!("{}", x));
        });
    }
    Ok(())
}

fn handle_stream<const SIZE: usize>(mut stream: TcpStream) -> Result<(), io::Error> {
    let received = read_stream::<SIZE>(&mut stream)?;

    let get = b"GET / HTTP/1.1\r\n";

    let good_response = make_ok_response()?;
    let not_found = make_not_found_response()?;
    let response =
        if received.starts_with(get) { good_response.as_bytes() } else { not_found.as_bytes() };
    write_stream(stream, response)
}

fn read_stream<const SIZE: usize>(stream: &mut TcpStream) -> Result<[u8; SIZE], io::Error> {
    let mut buffer = [0; SIZE];
    stream.read(&mut buffer)?;
    Ok(buffer)
}

fn write_stream(mut stream: TcpStream, response: &[u8]) -> Result<(), io::Error> {
    stream.write(&response)?;
    stream.flush()
}

fn make_response_from_file(code: u16, status_description: &str, file_name: &str) -> Result<String, io::Error> {
    let status_line = format!("HTTP/1.1 {code} {status_description}");
    let contents = fs::read_to_string(format!("templates/{file_name}"))?;
    Ok(format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    ))
}

fn make_ok_response() -> Result<String, io::Error> {
    make_response_from_file(200, "OK", "hello.html")
}

fn make_not_found_response() -> Result<String, io::Error> {
    make_response_from_file(404, "Not found", "404.html")
}
