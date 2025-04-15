use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn write_response_no_body(
    stream: &mut TcpStream,
    status_code: &str,
    reason_phrase: &str,
) -> io::Result<()> {
    write_response(stream, status_code, reason_phrase, None)
}

fn write_response(
    stream: &mut TcpStream,
    status_code: &str,
    reason_phrase: &str,
    body: Option<&str>,
) -> io::Result<()> {
    let http_version = "HTTP/1.1";
    write!(
        stream,
        "{http_version} {status_code} {reason_phrase}\r\n",
    )?;

    match body {
        Some(body) => {
            // Content length header
            write!(stream, "Content-Length: {len}\r\n\r\n", len = body.len())?;
         
            write!(stream, "{body}")
        }
        None => write!(stream, "\r\n"),
    }
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = vec![0_u8; 1024];

    let read_len = stream.read(&mut buffer)?;
    println!("Read some bytes: {read_len}");

    let request = std::str::from_utf8(&buffer).unwrap();
    println!("Read this data: {request}");
    let mut lines = request.lines();
    let request_line = lines.next().unwrap();
    let request_parts = request_line.split_whitespace().collect::<Vec<_>>();

    let [method, uri, http_version] = request_parts.as_slice() else {
        panic!("we'll sort this out later");
    };

    dbg!(method);
    dbg!(uri);
    dbg!(http_version);

    if *http_version != "HTTP/1.1" {
        return write_response_no_body(&mut stream, "505", "HTTP Version not supported");
    }

    if *method != "GET" {
        return write_response_no_body(&mut stream, "405", "Method Not Allowed");
    }

    if *uri != "/hello" {
        return write_response_no_body(&mut stream, "404", "Not Found");
    }

    write_response(&mut stream, "200", "Rust for Lunch", Some("Hello, World!"))
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?).unwrap();
    }
    Ok(())
}
